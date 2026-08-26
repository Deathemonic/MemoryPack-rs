use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

mod attributes;
mod circular;
mod enums;
mod helpers;
mod regular;
mod unions;
mod unmanaged;
mod version_tolerant;

use attributes::AttributeFlags;
use circular::{generate_circular_deserialize, generate_circular_serialize};
use enums::{
    generate_enum_deserialize_safe,
    generate_enum_deserialize_unsafe,
    generate_enum_serialize,
    generate_flags_impls,
    generate_transparent_deserialize,
    generate_transparent_serialize
};
use helpers::{generate_size_hint, has_explicit_discriminants, has_repr_c, is_single_field_i32};
use regular::{generate_deserialize, generate_serialize};
use unions::{generate_union_deserialize, generate_union_serialize};
use version_tolerant::{
    generate_version_tolerant_deserialize,
    generate_version_tolerant_serialize
};

#[proc_macro_derive(MemoryPackable, attributes(memorypack, tag))]
pub fn derive_memorypack(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let attrs = AttributeFlags::parse(&input.attrs);
    let (serialize_impl, deserialize_impl) = match generate_impls(&input, &attrs) {
        Ok(impls) => impls,
        Err(error) => return error.to_compile_error().into()
    };

    let flags_impl = if attrs.is_flags && attrs.is_transparent {
        generate_flags_impls(name)
    } else {
        quote! {}
    };

    let size_hint_impl = match &input.data {
        Data::Struct(data) if has_repr_c(&input.attrs) => unmanaged::size_hint(data),
        _ => generate_size_hint(
            &input.data,
            attrs.is_union,
            attrs.is_version_tolerant,
            attrs.is_circular
        )
    };

    let zero_copy_impl = if attrs.is_zero_copy {
        quote! {
            impl<'a> memorypack::MemoryPackDeserializeZeroCopy<'a> for #name<'a> {
                #[inline]
                fn deserialize(reader: &mut memorypack::MemoryPackReader<'a>) -> Result<Self, memorypack::MemoryPackError> {
                    #deserialize_impl
                }
            }
        }
    } else {
        quote! {}
    };

    let deserialize_regular_impl = if attrs.is_zero_copy {
        quote! {}
    } else {
        quote! {
            impl #impl_generics memorypack::MemoryPackDeserialize for #name #ty_generics #where_clause {
                #[inline]
                fn deserialize(reader: &mut memorypack::MemoryPackReader) -> Result<Self, memorypack::MemoryPackError> {
                    #deserialize_impl
                }
            }
        }
    };

    quote! {
        impl #impl_generics memorypack::MemoryPackSerialize for #name #ty_generics #where_clause {
            #[inline]
            fn serialize(&self, writer: &mut memorypack::MemoryPackWriter) -> Result<(), memorypack::MemoryPackError> {
                #serialize_impl
                Ok(())
            }

            #[inline]
            fn serialized_size_hint(&self) -> usize {
                #size_hint_impl
            }
        }

        #deserialize_regular_impl

        #zero_copy_impl

        #flags_impl
    }
    .into()
}

fn generate_impls(
    input: &DeriveInput,
    attrs: &AttributeFlags
) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
    let name = &input.ident;
    let impls = match &input.data {
        Data::Struct(data_struct) if has_repr_c(&input.attrs) => {
            (unmanaged::serialize(data_struct), unmanaged::deserialize(data_struct))
        }
        Data::Struct(data_struct) if attrs.is_transparent && is_single_field_i32(data_struct) => {
            (generate_transparent_serialize(), generate_transparent_deserialize())
        }
        Data::Struct(_) if attrs.is_circular => (
            generate_circular_serialize(&input.data, true),
            generate_circular_deserialize(&input.data, true)
        ),
        Data::Struct(_) if attrs.is_version_tolerant => (
            generate_version_tolerant_serialize(&input.data),
            generate_version_tolerant_deserialize(&input.data)
        ),
        Data::Struct(_) => {
            (generate_serialize(&input.data), generate_deserialize(&input.data, attrs.is_zero_copy))
        }
        Data::Enum(data_enum) if attrs.is_union => {
            (generate_union_serialize(data_enum), generate_union_deserialize(name, data_enum))
        }
        Data::Enum(data_enum) => {
            let has_explicit = has_explicit_discriminants(data_enum);

            if !attrs.has_repr_i32 && !has_explicit {
                return Err(syn::Error::new_spanned(
                    input,
                    "C-like enums for MemoryPack must have either #[repr(i32)] or explicit discriminants"
                ));
            }

            let deserialize = if has_explicit {
                generate_enum_deserialize_safe(data_enum)
            } else {
                generate_enum_deserialize_unsafe()
            };

            (generate_enum_serialize(), deserialize)
        }
        Data::Union(_) => {
            return Err(syn::Error::new_spanned(
                input,
                "MemoryPackable cannot be derived for Rust unions"
            ));
        }
    };
    Ok(impls)
}
