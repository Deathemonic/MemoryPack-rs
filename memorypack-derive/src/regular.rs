use quote::quote;
use syn::{Data, Fields};
use crate::helpers::{should_skip_field, prepare_ordered_fields};

pub fn generate_serialize(data: &Data) -> proc_macro2::TokenStream {
    let Data::Struct(data_struct) = data else {
        return quote! {
            compile_error!("MemoryPackable serialize can only be derived for structs");
        };
    };

    match &data_struct.fields {
        Fields::Named(fields) => {
            let non_skip: Vec<_> = fields.named.iter().filter(|f| !should_skip_field(f)).collect();
            let ordered = prepare_ordered_fields(&non_skip);
            let field_count = ordered.len() as u8;

            let serialize_fields = ordered.iter().map(|of| {
                let name = of.ident;
                quote! { memorypack::MemoryPackSerialize::serialize(&self.#name, writer)?; }
            });

            quote! {
                writer.write_u8(#field_count)?;
                #(#serialize_fields)*
            }
        }
        Fields::Unnamed(fields) => {
            let field_count = fields.unnamed.len() as u8;
            let serialize_fields = (0..fields.unnamed.len()).map(|i| {
                let index = syn::Index::from(i);
                quote! { memorypack::MemoryPackSerialize::serialize(&self.#index, writer)?; }
            });

            quote! {
                writer.write_u8(#field_count)?;
                #(#serialize_fields)*
            }
        }
        Fields::Unit => quote! {},
    }
}

pub fn generate_deserialize(data: &Data) -> proc_macro2::TokenStream {
    let Data::Struct(data_struct) = data else {
        return quote! {
            compile_error!("MemoryPackable deserialize can only be derived for structs");
        };
    };

    match &data_struct.fields {
        Fields::Named(fields) => {
            let non_skip: Vec<_> = fields.named.iter().filter(|f| !should_skip_field(f)).collect();
            let ordered = prepare_ordered_fields(&non_skip);
            
            let all_field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
            
            let deserialize_stmts: Vec<_> = fields.named.iter().map(|f| {
                let name = &f.ident;
                if should_skip_field(f) {
                    let ty = &f.ty;
                    quote! {
                        let _: #ty = memorypack::MemoryPackDeserialize::deserialize(reader)?;
                        let #name = Default::default();
                    }
                } else {
                    quote! { let #name = memorypack::MemoryPackDeserialize::deserialize(reader)?; }
                }
            }).collect();

            let mut ordered_deserialize = Vec::new();
            let mut skip_field_idx = 0;
            let mut ordered_idx = 0;
            
            for f in &fields.named {
                if should_skip_field(f) {
                    ordered_deserialize.push(deserialize_stmts[skip_field_idx + ordered_idx].clone());
                    skip_field_idx += 1;
                } else if ordered_idx < ordered.len() {
                    let field_idx = fields.named.iter()
                        .position(|field| std::ptr::eq(field, ordered[ordered_idx].field))
                        .unwrap();
                    ordered_deserialize.push(deserialize_stmts[field_idx].clone());
                    ordered_idx += 1;
                }
            }

            quote! {
                let _header = reader.read_u8()?;
                #(#ordered_deserialize)*
                Ok(Self { #(#all_field_names),* })
            }
        }
        Fields::Unnamed(fields) => {
            let len = fields.unnamed.len();
            let field_vars: Vec<_> = (0..len)
                .map(|i| syn::Ident::new(&format!("field_{}", i), proc_macro2::Span::call_site()))
                .collect();

            let deserialize_stmts = field_vars.iter().map(|var| {
                quote! { let #var = memorypack::MemoryPackDeserialize::deserialize(reader)?; }
            });

            quote! {
                let _header = reader.read_u8()?;
                #(#deserialize_stmts)*
                Ok(Self(#(#field_vars),*))
            }
        }
        Fields::Unit => quote! { Ok(Self) },
    }
}

