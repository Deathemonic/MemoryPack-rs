use quote::quote;
use syn::{Data, Fields};
use crate::helpers::{should_skip_field, prepare_ordered_fields, is_option_box};

pub fn generate_circular_serialize(data: &Data, needs_state: bool) -> proc_macro2::TokenStream {
    let Data::Struct(data_struct) = data else {
        return quote! {
            compile_error!("MemoryPackable circular can only be derived for structs");
        };
    };

    match &data_struct.fields {
        Fields::Named(fields) => {
            let non_skip: Vec<_> = fields.named.iter().filter(|f| !should_skip_field(f)).collect();
            let ordered = prepare_ordered_fields(&non_skip);
            let max_order = ordered.last().map(|f| f.order).unwrap_or(0);
            let member_count = max_order + 1;

            let field_serialization: Vec<_> = ordered.iter().map(|of| {
                let name = of.ident;
                let field = of.field;
                
                if is_option_box(&field.ty) {
                    quote! {
                        let mut temp_writer = memorypack::MemoryPackWriter::new();
                        match &self.#name {
                            None => {
                                temp_writer.write_u8(255)?;
                            }
                            Some(boxed_value) => {
                                memorypack::MemoryPackSerialize::serialize(&**boxed_value, &mut temp_writer)?;
                            }
                        }
                        field_buffers.push(temp_writer.buffer);
                    }
                } else {
                    quote! {
                        let mut temp_writer = memorypack::MemoryPackWriter::new();
                        memorypack::MemoryPackSerialize::serialize(&self.#name, &mut temp_writer)?;
                        field_buffers.push(temp_writer.buffer);
                    }
                }
            }).collect();

            let serialize_body = if needs_state {
                quote! {
                    if writer.optional_state.is_none() {
                        writer.optional_state = Some(memorypack::MemoryPackWriterOptionalState::new());
                    }
                    
                    let (is_existing, ref_id) = writer.optional_state.as_mut().unwrap().get_or_add_reference(self);
                    
                    if is_existing {
                        writer.write_u8(250)?;
                        memorypack::varint::write_varint(writer, ref_id as i64)?;
                        return Ok(());
                    }
                    
                    writer.write_u8(#member_count as u8)?;
                    let mut field_buffers: Vec<Vec<u8>> = Vec::new();
                    #(#field_serialization)*
                    
                    for i in 0..#member_count {
                        if i < field_buffers.len() {
                            memorypack::varint::write_varint(writer, field_buffers[i].len() as i64)?;
                        } else {
                            memorypack::varint::write_varint(writer, 0)?;
                        }
                    }
                    
                    memorypack::varint::write_varint(writer, ref_id as i64)?;
                    
                    for buf in field_buffers {
                        writer.buffer.extend_from_slice(&buf);
                    }
                }
            } else {
                quote! {
                    writer.write_u8(#member_count as u8)?;
                    let mut field_buffers: Vec<Vec<u8>> = Vec::new();
                    #(#field_serialization)*
                    
                    for i in 0..#member_count {
                        if i < field_buffers.len() {
                            memorypack::varint::write_varint(writer, field_buffers[i].len() as i64)?;
                        } else {
                            memorypack::varint::write_varint(writer, 0)?;
                        }
                    }
                    
                    memorypack::varint::write_varint(writer, 0)?;
                    
                    for buf in field_buffers {
                        writer.buffer.extend_from_slice(&buf);
                    }
                }
            };

            serialize_body
        }
        Fields::Unnamed(fields) => {
            let field_count = fields.unnamed.len();
            let field_indices: Vec<_> = (0..field_count).collect();

            let field_serialization: Vec<_> = field_indices.iter().map(|i| {
                let idx = syn::Index::from(*i);
                let field = &fields.unnamed[*i];
                
                if is_option_box(&field.ty) {
                    quote! {
                        let mut temp_writer = memorypack::MemoryPackWriter::new();
                        match &self.#idx {
                            None => {
                                temp_writer.write_u8(255)?;
                            }
                            Some(boxed_value) => {
                                memorypack::MemoryPackSerialize::serialize(&**boxed_value, &mut temp_writer)?;
                            }
                        }
                        field_buffers.push(temp_writer.buffer);
                    }
                } else {
                    quote! {
                        let mut temp_writer = memorypack::MemoryPackWriter::new();
                        memorypack::MemoryPackSerialize::serialize(&self.#idx, &mut temp_writer)?;
                        field_buffers.push(temp_writer.buffer);
                    }
                }
            }).collect();

            if needs_state {
                quote! {
                    if writer.optional_state.is_none() {
                        writer.optional_state = Some(memorypack::MemoryPackWriterOptionalState::new());
                    }
                    
                    let (is_existing, ref_id) = writer.optional_state.as_mut().unwrap().get_or_add_reference(self);
                    
                    if is_existing {
                        writer.write_u8(250)?;
                        memorypack::varint::write_varint(writer, ref_id as i64)?;
                        return Ok(());
                    }
                    
                    writer.write_u8(#field_count as u8)?;
                    let mut field_buffers: Vec<Vec<u8>> = Vec::new();
                    #(#field_serialization)*
                    
                    for buf in &field_buffers {
                        memorypack::varint::write_varint(writer, buf.len() as i64)?;
                    }
                    
                    memorypack::varint::write_varint(writer, ref_id as i64)?;
                    
                    for buf in field_buffers {
                        writer.buffer.extend_from_slice(&buf);
                    }
                }
            } else {
                quote! {
                    writer.write_u8(#field_count as u8)?;
                    let mut field_buffers: Vec<Vec<u8>> = Vec::new();
                    #(#field_serialization)*
                    
                    for buf in &field_buffers {
                        memorypack::varint::write_varint(writer, buf.len() as i64)?;
                    }
                    
                    memorypack::varint::write_varint(writer, 0)?;
                    
                    for buf in field_buffers {
                        writer.buffer.extend_from_slice(&buf);
                    }
                }
            }
        }
        Fields::Unit => quote! {
            writer.write_u8(0)?;
        },
    }
}

pub fn generate_circular_deserialize(data: &Data, needs_state: bool) -> proc_macro2::TokenStream {
    let _ = needs_state;
    let Data::Struct(data_struct) = data else {
        return quote! {
            compile_error!("MemoryPackable circular can only be derived for structs");
        };
    };

    match &data_struct.fields {
        Fields::Named(fields) => {
            let non_skip: Vec<_> = fields.named.iter().filter(|f| !should_skip_field(f)).collect();
            
            if non_skip.is_empty() {
                return quote! {
                    if reader.optional_state.is_none() {
                        reader.optional_state = Some(memorypack::MemoryPackReaderOptionalState::new());
                    }
                    
                    let member_count_or_ref = reader.read_u8()?;
                    if member_count_or_ref == 250 {
                        let ref_id = memorypack::varint::read_varint(reader)? as u32;
                        return reader.optional_state.as_ref().unwrap().get_object_reference::<Self>(ref_id);
                    }
                    let result = Self {};
                    let ref_id = memorypack::varint::read_varint(reader)? as u32;
                    reader.optional_state.as_mut().unwrap().add_object_reference(ref_id, result.clone())?;
                    Ok(result)
                };
            }

            let ordered = prepare_ordered_fields(&non_skip);
            let all_field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();

            let deserialize_logic: Vec<_> = ordered.iter().map(|of| {
                let name = of.ident;
                let order = of.order;
                let field = of.field;
                
                if is_option_box(&field.ty) {
                    quote! {
                        let #name = if #order < member_count && lengths[#order] > 0 {
                            let first_byte = reader.read_u8()?;
                            if first_byte == 255 {
                                None
                            } else {
                                reader.rewind(1)?;
                                Some(Box::new(memorypack::MemoryPackDeserialize::deserialize(reader)?))
                            }
                        } else {
                            if #order < member_count {
                                reader.skip(lengths[#order])?;
                            }
                            None
                        };
                    }
                } else {
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

            quote! {
                if reader.optional_state.is_none() {
                    reader.optional_state = Some(memorypack::MemoryPackReaderOptionalState::new());
                }
                
                let member_count_or_ref = reader.read_u8()?;
                
                if member_count_or_ref == 250 {
                    let ref_id = memorypack::varint::read_varint(reader)? as u32;
                    return reader.optional_state.as_ref().unwrap().get_object_reference::<Self>(ref_id);
                }
                
                let member_count = member_count_or_ref as usize;
                let mut lengths = Vec::with_capacity(member_count);
                for _ in 0..member_count {
                    lengths.push(memorypack::varint::read_varint(reader)? as usize);
                }
                let ref_id = memorypack::varint::read_varint(reader)? as u32;
                
                let placeholder = Self { #(#all_field_names: Default::default()),* };
                reader.optional_state.as_mut().unwrap().add_object_reference(ref_id, placeholder)?;
                
                #(#deserialize_logic)*
                #skip_extra_fields
                
                let result = Self { #(#all_field_names),* };
                reader.optional_state.as_mut().unwrap().update_object_reference(ref_id, result.clone())?;
                
                Ok(result)
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
                if reader.optional_state.is_none() {
                    reader.optional_state = Some(memorypack::MemoryPackReaderOptionalState::new());
                }
                
                let member_count_or_ref = reader.read_u8()?;
                
                if member_count_or_ref == 250 {
                    let ref_id = memorypack::varint::read_varint(reader)? as u32;
                    return reader.optional_state.as_ref().unwrap().get_object_reference::<Self>(ref_id);
                }
                
                let member_count = member_count_or_ref as usize;
                let mut lengths = Vec::with_capacity(member_count);
                for _ in 0..member_count {
                    lengths.push(memorypack::varint::read_varint(reader)? as usize);
                }
                let ref_id = memorypack::varint::read_varint(reader)? as u32;
                
                #(#deserialize_fields)*
                for i in #field_count..member_count {
                    reader.skip(lengths[i])?;
                }
                
                let result = Self(#(#field_vars),*);
                reader.optional_state.as_mut().unwrap().add_object_reference(ref_id, result.clone())?;
                
                Ok(result)
            }
        }
        Fields::Unit => quote! {
            if reader.optional_state.is_none() {
                reader.optional_state = Some(memorypack::MemoryPackReaderOptionalState::new());
            }
            
            let member_count_or_ref = reader.read_u8()?;
            if member_count_or_ref == 250 {
                let ref_id = memorypack::varint::read_varint(reader)? as u32;
                return reader.optional_state.as_ref().unwrap().get_object_reference::<Self>(ref_id);
            }
            let result = Self;
            let ref_id = memorypack::varint::read_varint(reader)? as u32;
            reader.optional_state.as_mut().unwrap().add_object_reference(ref_id, result.clone())?;
            Ok(result)
        },
    }
}

