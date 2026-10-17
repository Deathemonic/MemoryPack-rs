use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DataStruct, Fields, Member};

use crate::generation::helpers::{get_field_order, should_skip_field};
const MAX_MEMBER_COUNT: usize = 249;
const MAX_INLINE_LENGTH: usize = 127;
const REFERENCE_HEADER: u8 = 250;

struct FieldLayout {
    member: Member,
    binding: syn::Ident,
    order: usize,
    skipped: bool
}

pub fn generate_version_tolerant_serialize(data: &Data) -> TokenStream {
    generate_serialize(data, false)
}

pub fn generate_version_tolerant_deserialize(data: &Data) -> TokenStream {
    generate_deserialize(data, false)
}

pub fn generate_serialize(data: &Data, circular: bool) -> TokenStream {
    let Data::Struct(structure) = data else {
        return quote! { compile_error!("MemoryPackable requires a struct"); };
    };

    let fields = field_layout(structure);
    let member_count = member_count(&fields);

    if member_count > MAX_MEMBER_COUNT {
        return quote! { compile_error!("MemoryPack objects support at most 249 members"); };
    }

    let reference_header = serialize_reference(circular);
    let reference_id = circular.then(|| {
        quote! {
            memorypack::varint::write_varint(writer, i64::from(reference_id))?;
        }
    });
    let writes = fields
        .iter()
        .filter(|field| !field.skipped)
        .map(serialize_field);

    quote! {
        #reference_header
        writer.write_u8(#member_count as u8)?;

        let lengths_position = writer.buffer.len();
        writer.buffer.resize(lengths_position + #member_count, 0);
        #reference_id
        let mut field_lengths = [0_usize; #member_count];
        #(#writes)*

        if field_lengths.iter().all(|&length| length <= #MAX_INLINE_LENGTH) {
            for (index, length) in field_lengths.into_iter().enumerate() {
                writer.buffer[lengths_position + index] = length as u8;
            }
        } else {
            let mut length_writer = memorypack::MemoryPackWriter::new();

            for length in field_lengths {
                memorypack::varint::write_varint(&mut length_writer, length as i64)?;
            }

            writer.buffer.splice(lengths_position..lengths_position + #member_count, length_writer.buffer);
        }
    }
}

pub fn generate_deserialize(data: &Data, circular: bool) -> TokenStream {
    let Data::Struct(structure) = data else {
        return quote! { compile_error!("MemoryPackable requires a struct"); };
    };

    let fields = field_layout(structure);
    let bindings = fields
        .iter()
        .map(|field| &field.binding)
        .collect::<Vec<_>>();
    let reads = fields
        .iter()
        .filter(|field| !field.skipped)
        .map(deserialize_field);
    let construct = construct_value(structure, &fields);
    let reference_header = deserialize_reference(circular);
    let reference_register = circular.then(|| {
        quote! {
            let reference_id = memorypack::varint::read_varint(reader)? as u32;
            let placeholder = #construct;
            reader.optional_state.get_or_insert_with(memorypack::MemoryPackReaderOptionalState::new)
                .add_object_reference(reference_id, placeholder)?;
            #(let mut #bindings = Default::default();)*
        }
    });
    let reference_update = circular.then(|| {
        quote! {
            reader.optional_state.get_or_insert_with(memorypack::MemoryPackReaderOptionalState::new)
                .update_object_reference(reference_id, result.clone())?;
        }
    });

    quote! {
        let header = reader.read_u8()?;
        #reference_header

        if usize::from(header) > #MAX_MEMBER_COUNT {
            return Err(memorypack::MemoryPackError::InvalidLength(i32::from(header)));
        }

        let mut field_lengths = [0_usize; #MAX_MEMBER_COUNT];
        let lengths = &mut field_lengths[..usize::from(header)];

        for length in lengths.iter_mut() {
            *length = usize::try_from(memorypack::varint::read_varint(reader)?)
                .map_err(|_| memorypack::MemoryPackError::UnexpectedEndOfBuffer)?;
        }

        #(let mut #bindings = Default::default();)*
        #reference_register

        for (order, &length) in lengths.iter().enumerate() {
            let start = reader.position();

            match order {
                #(#reads)*
                _ => {}
            }

            let consumed = (reader.position() - start) as usize;
            reader.skip(length.checked_sub(consumed)
                .ok_or(memorypack::MemoryPackError::UnexpectedEndOfBuffer)?)?;
        }

        let result = #construct;
        #reference_update

        Ok(result)
    }
}

fn field_layout(structure: &DataStruct) -> Vec<FieldLayout> {
    structure
        .fields
        .iter()
        .enumerate()
        .map(|(index, field)| {
            let member = field
                .ident
                .clone()
                .map_or_else(|| Member::Unnamed(index.into()), Member::Named);

            FieldLayout {
                member,
                binding: format_ident!("field_{index}"),
                order: get_field_order(field).unwrap_or(index),
                skipped: should_skip_field(field)
            }
        })
        .collect()
}

fn member_count(fields: &[FieldLayout]) -> usize {
    fields
        .iter()
        .filter(|field| !field.skipped)
        .map(|field| field.order + 1)
        .max()
        .unwrap_or(0)
}

fn serialize_field(field: &FieldLayout) -> TokenStream {
    let FieldLayout { member, order, .. } = field;

    quote! {
        let start = writer.buffer.len();
        memorypack::MemoryPackSerialize::serialize(&self.#member, writer)?;
        field_lengths[#order] = writer.buffer.len() - start;
    }
}

fn deserialize_field(field: &FieldLayout) -> TokenStream {
    let FieldLayout { binding, order, .. } = field;

    quote! {
        #order if length > 0 => {
            #binding = memorypack::MemoryPackDeserialize::deserialize(reader)?;
        }
    }
}

fn construct_value(structure: &DataStruct, fields: &[FieldLayout]) -> TokenStream {
    let bindings = fields.iter().map(|field| &field.binding);

    match &structure.fields {
        Fields::Named(_) => {
            let members = fields.iter().map(|field| &field.member);

            quote! { Self { #(#members: #bindings),* } }
        }
        Fields::Unnamed(_) => quote! { Self(#(#bindings),*) },
        Fields::Unit => quote! { Self }
    }
}

fn serialize_reference(circular: bool) -> TokenStream {
    if !circular {
        return quote! {};
    }

    quote! {
        let (existing, reference_id) = writer.optional_state
            .get_or_insert_with(memorypack::MemoryPackWriterOptionalState::new)
            .get_or_add_reference(self);

        if existing {
            writer.write_object_reference_id(reference_id)?;

            return Ok(());
        }
    }
}

fn deserialize_reference(circular: bool) -> TokenStream {
    if !circular {
        return quote! {};
    }

    quote! {
        if header == #REFERENCE_HEADER {
            let reference_id = memorypack::varint::read_varint(reader)? as u32;

            return reader.optional_state
                .get_or_insert_with(memorypack::MemoryPackReaderOptionalState::new)
                .get_object_reference(reference_id);
        }
    }
}
