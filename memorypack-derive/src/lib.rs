use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

mod attributes;
mod helpers;
mod regular;
mod enums;
mod unions;
mod version_tolerant;
mod circular;

use attributes::AttributeFlags;
use helpers::{is_single_field_i32, has_explicit_discriminants};
use regular::{generate_serialize, generate_deserialize};
use enums::{
    generate_enum_serialize, generate_enum_deserialize_safe, generate_enum_deserialize_unsafe,
    generate_transparent_serialize, generate_transparent_deserialize, generate_flags_impls,
};
use unions::{generate_union_serialize, generate_union_deserialize};
use version_tolerant::{generate_version_tolerant_serialize, generate_version_tolerant_deserialize};
use circular::{generate_circular_serialize, generate_circular_deserialize};

#[proc_macro_derive(MemoryPackable, attributes(memorypack, tag))]
pub fn derive_memorypack(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let attrs = AttributeFlags::parse(&input.attrs);

    let (serialize_impl, deserialize_impl) = match &input.data {
        Data::Struct(data_struct) if attrs.is_transparent && is_single_field_i32(data_struct) => (
            generate_transparent_serialize(),
            generate_transparent_deserialize(),
        ),
        Data::Struct(_) if attrs.is_circular => (
            generate_circular_serialize(&input.data, true),
            generate_circular_deserialize(&input.data, true),
        ),
        Data::Struct(_) if attrs.is_version_tolerant => (
            generate_version_tolerant_serialize(&input.data),
            generate_version_tolerant_deserialize(&input.data),
        ),
        Data::Struct(_) => (
            generate_serialize(&input.data),
            generate_deserialize(&input.data),
        ),
        Data::Enum(data_enum) if attrs.is_union => (
            generate_union_serialize(data_enum),
            generate_union_deserialize(name, data_enum),
        ),
        Data::Enum(data_enum) => {
            let has_explicit = has_explicit_discriminants(data_enum);
            
            if !attrs.has_repr_i32 && !has_explicit {
                return syn::Error::new_spanned(
                    &input,
                    "C-like enums for MemoryPack must have either #[repr(i32)] or explicit discriminants"
                ).to_compile_error().into();
            }
            
            let deserialize = if has_explicit {
                generate_enum_deserialize_safe(data_enum)
            } else {
                generate_enum_deserialize_unsafe()
            };
            
            (generate_enum_serialize(), deserialize)
        }
        Data::Union(_) => {
            return syn::Error::new_spanned(
                &input,
                "MemoryPackable cannot be derived for Rust unions"
            ).to_compile_error().into();
        }
    };

    let flags_impl = if attrs.is_flags && attrs.is_transparent {
        generate_flags_impls(name)
    } else {
        quote! {}
    };

    let expanded = quote! {
        impl #impl_generics memorypack::MemoryPackSerialize for #name #ty_generics #where_clause {
            fn serialize(&self, writer: &mut memorypack::MemoryPackWriter) -> Result<(), memorypack::MemoryPackError> {
                #serialize_impl
                Ok(())
            }
        }

        impl #impl_generics memorypack::MemoryPackDeserialize for #name #ty_generics #where_clause {
            fn deserialize(reader: &mut memorypack::MemoryPackReader) -> Result<Self, memorypack::MemoryPackError> {
                #deserialize_impl
            }
        }

        #flags_impl
    };

    expanded.into()
}
