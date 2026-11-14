use quote::quote;
use syn::{Data, Fields};

use crate::generation::helpers::{
    DeserializationMode,
    generate_field_deserialize_value,
    generate_field_serialize,
    prepare_ordered_fields,
    should_skip_field
};

pub fn generate_serialize(data: &Data) -> proc_macro2::TokenStream {
    let Data::Struct(data_struct) = data else {
        return quote! {
            compile_error!("MemoryPackable serialize can only be derived for structs");
        };
    };

    match &data_struct.fields {
        Fields::Named(fields) => {
            let serialized_fields: Vec<_> = fields
                .named
                .iter()
                .filter(|f| !should_skip_field(f))
                .collect();
            let ordered = prepare_ordered_fields(&serialized_fields);
            let field_count = ordered.len() as u8;

            let serialize_fields = ordered.iter().map(|ordered_field| {
                let name = ordered_field.ident;

                generate_field_serialize(ordered_field.field, &quote! { &self.#name })
            });

            quote! {
                writer.write_u8(#field_count)?;
                #(#serialize_fields)*
            }
        }
        Fields::Unnamed(fields) => {
            let field_count = fields.unnamed.len() as u8;
            let serialize_fields = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, field)| {
                    let index = syn::Index::from(i);

                    generate_field_serialize(field, &quote! { &self.#index })
                });

            quote! {
                writer.write_u8(#field_count)?;
                #(#serialize_fields)*
            }
        }
        Fields::Unit => quote! { writer.write_u8(0)?; }
    }
}

pub fn generate_deserialize(data: &Data, mode: DeserializationMode) -> proc_macro2::TokenStream {
    let Data::Struct(data_struct) = data else {
        return quote! {
            compile_error!("MemoryPackable deserialize can only be derived for structs");
        };
    };

    match &data_struct.fields {
        Fields::Named(fields) => {
            let serialized_fields: Vec<_> = fields
                .named
                .iter()
                .filter(|f| !should_skip_field(f))
                .collect();
            let ordered = prepare_ordered_fields(&serialized_fields);
            let field_count = ordered.len() as u8;

            let all_field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();

            let skipped = fields
                .named
                .iter()
                .filter(|field| should_skip_field(field))
                .map(|field| {
                    let name = &field.ident;
                    quote! { let #name = Default::default(); }
                });
            let reads = ordered
                .iter()
                .enumerate()
                .map(|(index, ordered)| {
                    let name = ordered.ident;
                    let read = generate_field_deserialize_value(ordered.field, mode);
                    let position = index as u8;
                    quote! {
                        let #name = if header > #position { #read } else { Default::default() };
                    }
                });

            quote! {
                let header = reader.read_u8()?;
                if header > #field_count {
                    return Err(memorypack::MemoryPackError::InvalidObjectMemberCount {
                        name: std::any::type_name::<Self>(), expected: #field_count, actual: header
                    });
                }
                #(#skipped)*
                #(#reads)*
                Ok(Self { #(#all_field_names),* })
            }
        }
        Fields::Unnamed(fields) => {
            let len = fields.unnamed.len();
            let field_count = len as u8;
            let field_vars: Vec<_> = (0..len)
                .map(|i| syn::Ident::new(&format!("field_{i}"), proc_macro2::Span::call_site()))
                .collect();

            let deserialize_stmts = field_vars
                .iter()
                .zip(&fields.unnamed)
                .enumerate()
                .map(|(index, (var, field))| {
                    let deserialize_value = generate_field_deserialize_value(field, mode);

                    let position = index as u8;
                    quote! { let #var = if header > #position { #deserialize_value } else { Default::default() }; }
                });

            quote! {
                let header = reader.read_u8()?;
                if header > #field_count {
                    return Err(memorypack::MemoryPackError::InvalidObjectMemberCount {
                        name: std::any::type_name::<Self>(), expected: #field_count, actual: header
                    });
                }
                #(#deserialize_stmts)*
                Ok(Self(#(#field_vars),*))
            }
        }
        Fields::Unit => quote! {
            let header = reader.read_u8()?;
            if header != 0 {
                return Err(memorypack::MemoryPackError::InvalidObjectMemberCount {
                    name: std::any::type_name::<Self>(), expected: 0, actual: header
                });
            }
            Ok(Self)
        }
    }
}
