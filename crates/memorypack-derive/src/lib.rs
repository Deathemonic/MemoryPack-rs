mod generation;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

use crate::generation::attributes::AttributeFlags;
use crate::generation::circular::{
    generate_circular_deserialize,
    generate_circular_serialize,
    generate_graph_impl
};
use crate::generation::enums::{
    generate_enum_deserialize_safe,
    generate_enum_serialize,
    generate_flags_impls,
    generate_transparent_deserialize,
    generate_transparent_serialize
};
use crate::generation::helpers::{
    generate_size_hint,
    has_explicit_discriminants,
    has_repr_c,
    is_single_field_i32,
    prepare_deserialization_generics
};
use crate::generation::regular::{generate_deserialize, generate_serialize};
use crate::generation::unions::{generate_union_deserialize, generate_union_serialize};
use crate::generation::unmanaged;
use crate::generation::version_tolerant::{
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

    let graph_impl = attrs
        .is_circular
        .then(|| generate_graph_impl(&input));

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

    let (deserialization_generics, input_lifetime) =
        prepare_deserialization_generics(&input.generics);
    let (deserialization_impl_generics, _, deserialization_where_clause) =
        deserialization_generics.split_for_impl();
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

        impl #deserialization_impl_generics memorypack::MemoryPackDeserialize<#input_lifetime> for #name #ty_generics #deserialization_where_clause {
            #[inline]
            fn deserialize(reader: &mut memorypack::MemoryPackReader<#input_lifetime>) -> Result<Self, memorypack::MemoryPackError> {
                #deserialize_impl
            }
        }

        #flags_impl
        #graph_impl
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
        Data::Struct(_) if attrs.is_circular => {
            (generate_circular_serialize(&input.data), generate_circular_deserialize(&input.data))
        }
        Data::Struct(_) if attrs.is_version_tolerant => (
            generate_version_tolerant_serialize(&input.data),
            generate_version_tolerant_deserialize(&input.data)
        ),
        Data::Struct(_) => (generate_serialize(&input.data), generate_deserialize(&input.data)),
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

            let deserialize = generate_enum_deserialize_safe(data_enum);

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
