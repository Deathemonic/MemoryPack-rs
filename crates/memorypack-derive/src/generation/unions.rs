use std::collections::BTreeSet;

use quote::quote;
use syn::{DataEnum, Fields, LitInt};

pub fn generate_union_serialize(data_enum: &syn::DataEnum) -> proc_macro2::TokenStream {
    for variant in &data_enum.variants {
        match &variant.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {}
            _ => {
                return quote! {
                    compile_error!("Union variants must have exactly one unnamed field");
                };
            }
        }
    }

    let tags = match union_tags(data_enum) {
        Ok(tags) => tags,
        Err(error) => return error.to_compile_error()
    };
    let variants = data_enum
        .variants
        .iter()
        .zip(tags)
        .map(|(variant, tag)| {
            let variant_name = &variant.ident;
            let header = if tag < 250 {
                let tag = tag as u8;
                quote! { writer.write_u8(#tag)?; }
            } else {
                quote! { writer.write_u8(250)?; writer.write_u16(#tag)?; }
            };

            quote! {
                Self::#variant_name(inner) => {
                    #header
                    memorypack::MemoryPackSerialize::serialize(inner, writer)?;
                }
            }
        });

    quote! {
        match self {
            #(#variants)*
        }
    }
}

pub fn generate_union_deserialize(
    name: &syn::Ident,
    data_enum: &syn::DataEnum
) -> proc_macro2::TokenStream {
    let tags = match union_tags(data_enum) {
        Ok(tags) => tags,
        Err(error) => return error.to_compile_error()
    };
    let variants = data_enum
        .variants
        .iter()
        .zip(tags)
        .map(|(variant, tag_value)| {
            let variant_name = &variant.ident;

            quote! {
                #tag_value => {
                    let inner = memorypack::MemoryPackDeserialize::deserialize(reader)?;
                    Ok(Self::#variant_name(inner))
                }
            }
        });

    quote! {
        let marker = reader.read_u8()?;
        if marker > 250 {
            return Err(memorypack::MemoryPackError::UnknownUnionTag {
                tag: u16::from(marker), name: stringify!(#name)
            });
        }
        let tag = if marker == 250 { reader.read_u16()? } else { u16::from(marker) };
        match tag {
            #(#variants)*
            _ => Err(memorypack::MemoryPackError::UnknownUnionTag {
                tag,
                name: stringify!(#name)
            })
        }
    }
}

fn union_tags(data: &DataEnum) -> syn::Result<Vec<u16>> {
    let mut tags = Vec::new();
    let mut seen = BTreeSet::new();

    for (index, variant) in data.variants.iter().enumerate() {
        let attributes: Vec<_> = variant
            .attrs
            .iter()
            .filter(|attribute| attribute.path().is_ident("tag"))
            .collect();
        let tag = match attributes.as_slice() {
            [] => u16::try_from(index)
                .map_err(|_| syn::Error::new_spanned(variant, "Too many union variants"))?,
            [attribute] => attribute.parse_args::<LitInt>()?.base10_parse()?,
            _ => return Err(syn::Error::new_spanned(variant, "Duplicate union tag attribute"))
        };

        if !seen.insert(tag) {
            return Err(syn::Error::new_spanned(variant, "Duplicate union tag"));
        }

        tags.push(tag);
    }

    Ok(tags)
}
