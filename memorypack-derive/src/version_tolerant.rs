use quote::quote;
use syn::{Data, Fields};
use crate::helpers::{should_skip_field, prepare_ordered_fields};

pub fn generate_version_tolerant_serialize(data: &Data) -> proc_macro2::TokenStream {
    let Data::Struct(data_struct) = data else {
        return quote! {
            compile_error!("MemoryPackable version_tolerant can only be derived for structs");
        };
    };

    match &data_struct.fields {
        Fields::Named(fields) => {
            let non_skip: Vec<_> = fields.named.iter().filter(|f| !should_skip_field(f)).collect();

            if non_skip.is_empty() {
                return quote! { writer.write_u8(0)?; };
            }

            let ordered = prepare_ordered_fields(&non_skip);
            let max_order = ordered.last().map(|f| f.order).unwrap_or(0);
            let member_count = max_order + 1;

            if member_count <= 3 {
                let serialize_fields_array = (0..member_count).map(|order| {
                    if let Some(of) = ordered.iter().find(|f| f.order == order) {
                        let name = of.ident;
                        quote! {
                            let start_pos = temp_buf.len();
                            memorypack::MemoryPackSerialize::serialize(&self.#name, &mut temp_buf)?;
                            field_lengths[#order] = (temp_buf.len() - start_pos) as i64;
                        }
                    } else {
                        quote! { field_lengths[#order] = 0i64; }
                    }
                });

                quote! {
                    writer.write_u8(#member_count as u8)?;
                    
                    let mut temp_buf = memorypack::MemoryPackWriter::with_capacity(64);
                    let mut field_lengths = [0i64; #member_count];
                    
                    #(#serialize_fields_array)*
                    
                    for &length in &field_lengths {
                        memorypack::varint::write_varint(writer, length)?;
                    }
                    
                    writer.buffer.extend_from_slice(&temp_buf.buffer);
                }
            } else {
                let serialize_to_temp = (0..member_count).map(|order| {
                    if let Some(of) = ordered.iter().find(|f| f.order == order) {
                        let name = of.ident;
                        quote! {
                            let start_pos = temp_buf.len();
                            memorypack::MemoryPackSerialize::serialize(&self.#name, &mut temp_buf)?;
                            let field_len = temp_buf.len() - start_pos;
                            field_lengths.push(field_len as i64);
                        }
                    } else {
                        quote! { field_lengths.push(0i64); }
                    }
                });

                quote! {
                    writer.write_u8(#member_count as u8)?;
                    
                    let mut temp_buf = memorypack::MemoryPackWriter::with_capacity(128);
                    let mut field_lengths = Vec::with_capacity(#member_count);
                    
                    #(#serialize_to_temp)*
                    
                    for &length in &field_lengths {
                        memorypack::varint::write_varint(writer, length)?;
                    }
                    
                    writer.buffer.extend_from_slice(&temp_buf.buffer);
                }
            }
        }
        Fields::Unnamed(fields) => {
            let field_count = fields.unnamed.len();
            let field_indices: Vec<_> = (0..field_count).map(syn::Index::from).collect();

            let serialize_to_temp = field_indices.iter().map(|idx| {
                quote! {
                    let start_pos = temp_buf.len();
                    memorypack::MemoryPackSerialize::serialize(&self.#idx, &mut temp_buf)?;
                    let field_len = temp_buf.len() - start_pos;
                    field_lengths.push(field_len as i64);
                }
            });

            quote! {
                writer.write_u8(#field_count as u8)?;
                
                let mut temp_buf = memorypack::MemoryPackWriter::with_capacity(64);
                let mut field_lengths = Vec::with_capacity(#field_count);
                
                #(#serialize_to_temp)*
                
                for &length in &field_lengths {
                    memorypack::varint::write_varint(writer, length)?;
                }
                
                writer.buffer.extend_from_slice(&temp_buf.buffer);
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
            let non_skip: Vec<_> = fields.named.iter().filter(|f| !should_skip_field(f)).collect();
            
            if non_skip.is_empty() {
                return quote! {
                    let _member_count = reader.read_u8()?;
                    Ok(Self {})
                };
            }

            let ordered = prepare_ordered_fields(&non_skip);
            let all_field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();

            let deserialize_logic: Vec<_> = ordered.iter().map(|of| {
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
            }).collect();

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

