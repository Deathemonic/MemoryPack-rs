use quote::quote;
use syn::Fields;

pub fn generate_union_serialize(data_enum: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = data_enum.variants.iter().enumerate().map(|(tag, variant)| {
        let variant_name = &variant.ident;
        let tag_value = tag as u8;

        match &variant.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                quote! {
                    Self::#variant_name(inner) => {
                        writer.write_u8(#tag_value)?;
                        memorypack::MemoryPackSerialize::serialize(inner, writer)?;
                    }
                }
            }
            _ => {
                return quote! {
                    compile_error!("Union variants must have exactly one unnamed field");
                };
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
    data_enum: &syn::DataEnum,
) -> proc_macro2::TokenStream {
    let variants = data_enum.variants.iter().enumerate().map(|(tag, variant)| {
        let variant_name = &variant.ident;
        let tag_value = tag as u8;

        quote! {
            #tag_value => {
                let inner = memorypack::MemoryPackDeserialize::deserialize(reader)?;
                Ok(Self::#variant_name(inner))
            }
        }
    });

    quote! {
        let tag = reader.read_u8()?;
        match tag {
            #(#variants)*
            _ => Err(memorypack::MemoryPackError::DeserializationError(
                format!("Unknown union tag {} for {}", tag, stringify!(#name))
            ))
        }
    }
}
