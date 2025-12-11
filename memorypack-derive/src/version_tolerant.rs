use crate::helpers::{prepare_ordered_fields, should_skip_field};

use quote::quote;
use syn::{Data, Fields};

pub fn generate_version_tolerant_serialize(data: &Data) -> proc_macro2::TokenStream {
    let Data::Struct(data_struct) = data else {
        return quote! {
            compile_error!("MemoryPackable version_tolerant can only be derived for structs");
        };
    };

    match &data_struct.fields {
        Fields::Named(fields) => {
            let non_skip: Vec<_> = fields
                .named
                .iter()
                .filter(|f| !should_skip_field(f))
                .collect();

            if non_skip.is_empty() {
                return quote! { writer.write_u8(0)?; };
            }

            let ordered = prepare_ordered_fields(&non_skip);
            let max_order = ordered.last().map(|f| f.order).unwrap_or(0);
            let member_count = max_order + 1;

            let serialize_fields: Vec<_> = (0..member_count)
                .map(|order| {
                    if let Some(of) = ordered.iter().find(|f| f.order == order) {
                        let name = of.ident;
                        quote! {
                            let start = writer.buffer.len();
                            memorypack::MemoryPackSerialize::serialize(&self.#name, writer)?;
                            field_lengths[#order] = writer.buffer.len() - start;
                        }
                    } else {
                        quote! { field_lengths[#order] = 0; }
                    }
                })
                .collect();

            let backpatch_assignments: Vec<_> = (0..member_count)
                .map(|i| {
                    quote! { writer.buffer[lengths_pos + #i] = field_lengths[#i] as u8; }
                })
                .collect();

            quote! {
                writer.write_u8(#member_count as u8)?;

                let lengths_pos = writer.buffer.len();
                writer.buffer.extend_from_slice(&[0u8; #member_count]);

                let mut field_lengths = [0usize; #member_count];
                #(#serialize_fields)*

                if field_lengths.iter().all(|&len| len < 128) {
                    #(#backpatch_assignments)*
                } else {
                    let data_start = lengths_pos + #member_count;
                    let data = writer.buffer[data_start..].to_vec();
                    writer.buffer.truncate(lengths_pos);
                    for &len in &field_lengths {
                        memorypack::varint::write_varint(writer, len as i64)?;
                    }
                    writer.buffer.extend_from_slice(&data);
                }
            }
        }
        Fields::Unnamed(fields) => {
            let field_count = fields.unnamed.len();
            let field_indices: Vec<_> = (0..field_count).map(syn::Index::from).collect();

            let serialize_fields: Vec<_> = field_indices
                .iter()
                .enumerate()
                .map(|(i, idx)| {
                    quote! {
                        let start = writer.buffer.len();
                        memorypack::MemoryPackSerialize::serialize(&self.#idx, writer)?;
                        field_lengths[#i] = writer.buffer.len() - start;
                    }
                })
                .collect();

            let backpatch_assignments: Vec<_> = (0..field_count)
                .map(|i| {
                    quote! { writer.buffer[lengths_pos + #i] = field_lengths[#i] as u8; }
                })
                .collect();

            quote! {
                writer.write_u8(#field_count as u8)?;

                let lengths_pos = writer.buffer.len();
                writer.buffer.extend_from_slice(&[0u8; #field_count]);

                let mut field_lengths = [0usize; #field_count];
                #(#serialize_fields)*

                if field_lengths.iter().all(|&len| len < 128) {
                    #(#backpatch_assignments)*
                } else {
                    let data_start = lengths_pos + #field_count;
                    let data = writer.buffer[data_start..].to_vec();
                    writer.buffer.truncate(lengths_pos);
                    for &len in &field_lengths {
                        memorypack::varint::write_varint(writer, len as i64)?;
                    }
                    writer.buffer.extend_from_slice(&data);
                }
            }
        }
        Fields::Unit => quote! { writer.write_u8(0)?; },
    }
}

pub fn generate_version_tolerant_deserialize(data: &Data) -> proc_macro2::TokenStream {
    let Data::Struct(data_struct) = data else {
        return quote! {
            compile_error!("MemoryPackable version_tolerant can only be derived for structs");
        };
    };

    match &data_struct.fields {
        Fields::Named(fields) => {
            let non_skip: Vec<_> = fields
                .named
                .iter()
                .filter(|f| !should_skip_field(f))
                .collect();

            if non_skip.is_empty() {
                return quote! {
                    let _member_count = reader.read_u8()?;
                    Ok(Self {})
                };
            }

            let ordered = prepare_ordered_fields(&non_skip);
            let all_field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();

            let deserialize_logic: Vec<_> = ordered
                .iter()
                .map(|of| {
                    let name = of.ident;
                    let order = of.order;
                    quote! {
                        let #name = if #order < member_count && lengths[#order] > 0 {
                            memorypack::MemoryPackDeserialize::deserialize(reader)?
                        } else {
                            if #order < member_count {
                                reader.skip(lengths[#order])?;
                            }
                            Default::default()
                        };
                    }
                })
                .collect();

            let skip_extra_fields = if let Some(max_order) = ordered.last().map(|f| f.order) {
                let next_order = max_order + 1;
                quote! {
                    for i in #next_order..member_count {
                        reader.skip(lengths[i])?;
                    }
                }
            } else {
                quote! {}
            };

            let max_fields = ordered.last().map(|f| f.order + 1).unwrap_or(0);

            if max_fields <= 8 {
                quote! {
                    let member_count = reader.read_u8()? as usize;
                    let mut lengths = [0usize; 8];
                    for i in 0..member_count.min(8) {
                        lengths[i] = memorypack::varint::read_varint(reader)? as usize;
                    }
                    #(#deserialize_logic)*
                    #skip_extra_fields
                    Ok(Self { #(#all_field_names),* })
                }
            } else {
                quote! {
                    let member_count = reader.read_u8()? as usize;
                    let mut lengths = Vec::with_capacity(member_count);
                    for _ in 0..member_count {
                        lengths.push(memorypack::varint::read_varint(reader)? as usize);
                    }
                    #(#deserialize_logic)*
                    #skip_extra_fields
                    Ok(Self { #(#all_field_names),* })
                }
            }
        }
        Fields::Unnamed(fields) => {
            let field_count = fields.unnamed.len();
            let field_vars: Vec<_> = (0..field_count)
                .map(|i| syn::Ident::new(&format!("field_{}", i), proc_macro2::Span::call_site()))
                .collect();

            let deserialize_fields = field_vars.iter().enumerate().map(|(i, var)| {
                quote! {
                    let #var = if #i < member_count {
                        memorypack::MemoryPackDeserialize::deserialize(reader)?
                    } else {
                        Default::default()
                    };
                }
            });

            quote! {
                let member_count = reader.read_u8()? as usize;
                let mut lengths = Vec::with_capacity(member_count);
                for _ in 0..member_count {
                    lengths.push(memorypack::varint::read_varint(reader)? as usize);
                }
                #(#deserialize_fields)*
                for i in #field_count..member_count {
                    reader.skip(lengths[i])?;
                }
                Ok(Self(#(#field_vars),*))
            }
        }
        Fields::Unit => quote! {
            let _member_count = reader.read_u8()?;
            Ok(Self)
        },
    }
}
