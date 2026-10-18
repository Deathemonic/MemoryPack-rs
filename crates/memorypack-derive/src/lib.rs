mod generation;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

use crate::generation::attributes::{AttributeFlags, FieldAttributes};
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
    DeserializationMode,
    generate_deserialization_impl,
    generate_size_hint,
    has_explicit_discriminants,
    has_repr_c,
    is_single_field_i32,
    prepare_serialization_generics,
    should_skip_field
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
    let (_, ty_generics, _) = input.generics.split_for_impl();
    let attrs = AttributeFlags::parse(&input.attrs);

    if let Err(error) = validate_formatter_attributes(&input, &attrs) {
        return error.to_compile_error().into();
    }

    if let Err(error) = validate_arena_attributes(&input, &attrs) {
        return error.to_compile_error().into();
    }

    let serialization_generics = prepare_serialization_generics(&input);
    let (serialization_impl_generics, _, serialization_where_clause) =
        serialization_generics.split_for_impl();

    let (serialize_impl, deserialize_body) = match generate_impls(&input, &attrs) {
        Ok(impls) => impls,
        Err(error) => return error.to_compile_error().into()
    };
    let deserialize_impl =
        generate_deserialization_impl(&input, DeserializationMode::Input, &deserialize_body);
    let arena_impl = generate_arena_impl(&input, &attrs);

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

    quote! {
        impl #serialization_impl_generics memorypack::MemoryPackSerialize for #name #ty_generics #serialization_where_clause {
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

        #deserialize_impl
        #arena_impl
        #flags_impl
        #graph_impl
    }
    .into()
}

fn generate_arena_impl(
    input: &DeriveInput,
    attributes: &AttributeFlags
) -> proc_macro2::TokenStream {
    if !attributes.is_arena {
        return quote! {};
    }

    let mode = DeserializationMode::Arena;
    let body = generate_deserialize(&input.data, mode);

    generate_deserialization_impl(input, mode, &body)
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
        Data::Struct(_) => (
            generate_serialize(&input.data),
            generate_deserialize(&input.data, DeserializationMode::Input)
        ),
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

fn validate_arena_attributes(input: &DeriveInput, attributes: &AttributeFlags) -> syn::Result<()> {
    if !attributes.is_arena {
        return Ok(());
    }

    if !matches!(input.data, Data::Struct(_))
        || attributes.is_circular
        || attributes.is_version_tolerant
        || attributes.is_transparent
        || has_repr_c(&input.attrs)
    {
        return Err(syn::Error::new_spanned(
            input,
            "arena deserialization supports regular structs"
        ));
    }

    Ok(())
}

fn validate_formatter_attributes(
    input: &DeriveInput,
    attributes: &AttributeFlags
) -> syn::Result<()> {
    if FieldAttributes::parse(&input.attrs)?
        .formatter
        .is_some()
    {
        return Err(syn::Error::new_spanned(
            input,
            "MemoryPack formatters must be applied to fields"
        ));
    }

    let fields: Vec<_> = match &input.data {
        Data::Struct(data) => data.fields.iter().collect(),
        Data::Enum(data) => {
            for variant in &data.variants {
                if FieldAttributes::parse(&variant.attrs)?
                    .formatter
                    .is_some()
                {
                    return Err(syn::Error::new_spanned(
                        variant,
                        "MemoryPack formatters must be applied to struct fields"
                    ));
                }
            }

            data.variants
                .iter()
                .flat_map(|variant| &variant.fields)
                .collect()
        }
        Data::Union(data) => data.fields.named.iter().collect()
    };

    for field in fields {
        if FieldAttributes::parse(&field.attrs)?
            .formatter
            .is_none()
        {
            continue;
        }

        validate_formatter_field(input, attributes, field)?;
    }

    Ok(())
}

fn validate_formatter_field(
    input: &DeriveInput,
    attributes: &AttributeFlags,
    field: &syn::Field
) -> syn::Result<()> {
    let Data::Struct(structure) = &input.data else {
        return Err(syn::Error::new_spanned(
            field,
            "MemoryPack formatters are supported on struct fields"
        ));
    };

    if has_repr_c(&input.attrs) || (attributes.is_transparent && is_single_field_i32(structure)) {
        return Err(syn::Error::new_spanned(
            field,
            "MemoryPack formatters cannot be used with unmanaged layouts"
        ));
    }

    if should_skip_field(field) {
        return Err(syn::Error::new_spanned(
            field,
            "MemoryPack formatters cannot be used on skipped fields"
        ));
    }

    Ok(())
}
