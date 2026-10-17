use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::error::MemoryPackError;
use crate::graph::arena::{GraphArena, ReferenceKey};
use crate::reader::MemoryPackReader;
use crate::serialization::format::OBJECT_REFERENCE_HEADER;
use crate::state::{MemoryPackReaderOptionalState, MemoryPackWriterOptionalState};
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::varint;
use crate::writer::MemoryPackWriter;

const MAX_MEMBER_COUNT: usize = 249;

pub struct MemoryPackReference<T> {
    pub(super) key: ReferenceKey,
    node_type: PhantomData<fn() -> T>
}

pub trait MemoryPackCircular: Sized {
    fn serialize_circular(
        &self,
        writer: &mut MemoryPackWriter,
        reference_id: u32
    ) -> Result<(), MemoryPackError>;
    fn deserialize_circular(
        reader: &mut MemoryPackReader,
        lengths: &[usize]
    ) -> Result<Self, MemoryPackError>;
}

impl<T> MemoryPackReference<T> {
    pub(super) const fn from_key(key: ReferenceKey) -> Self {
        Self {
            key,
            node_type: PhantomData
        }
    }
}

impl<T> Clone for MemoryPackReference<T> {
    fn clone(&self) -> Self { *self }
}

impl<T> Copy for MemoryPackReference<T> {}

impl<T> Default for MemoryPackReference<T> {
    fn default() -> Self { Self::from_key(ReferenceKey { arena: 0, index: 0 }) }
}

impl<T> fmt::Debug for MemoryPackReference<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result { self.key.fmt(formatter) }
}

impl<T> PartialEq for MemoryPackReference<T> {
    fn eq(&self, other: &Self) -> bool { self.key == other.key }
}

impl<T> Eq for MemoryPackReference<T> {}

impl<T: MemoryPackCircular + 'static> MemoryPackSerialize for MemoryPackReference<T> {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let state = writer
            .optional_state
            .get_or_insert_with(MemoryPackWriterOptionalState::new);
        let arena = state
            .graph
            .as_ref()
            .map(|context| Rc::clone(&context.arena))
            .ok_or(MemoryPackError::ObjectReferenceNotFound(self.key.error_id()))?;
        let value = arena.get::<T>(self.key)?;
        let (existing, reference_id) = state.get_or_add_graph_reference(self.key)?;

        if existing {
            return writer.write_object_reference_id(reference_id);
        }

        value.serialize_circular(writer, reference_id)
    }
}

impl<T: MemoryPackCircular + 'static> MemoryPackDeserialize for MemoryPackReference<T> {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        let header = reader.read_u8()?;

        if header == OBJECT_REFERENCE_HEADER {
            let reference_id = read_reference_id(reader)?;
            let key = reader_graph(reader, reference_id)?.resolve::<T>(reference_id)?;

            return Ok(Self::from_key(key));
        }

        let lengths = read_lengths(reader, header)?;
        let reference_id = read_reference_id(reader)?;
        let key = reader_graph(reader, reference_id)?.reserve::<T>(reference_id)?;
        let value = T::deserialize_circular(reader, &lengths)?;
        reader_graph(reader, reference_id)?.initialize(key, value)?;

        Ok(Self::from_key(key))
    }
}

fn reader_graph<'a>(
    reader: &'a mut MemoryPackReader,
    reference_id: u32
) -> Result<&'a mut GraphArena, MemoryPackError> {
    reader
        .optional_state
        .get_or_insert_with(MemoryPackReaderOptionalState::new)
        .graph
        .as_deref_mut()
        .ok_or(MemoryPackError::ObjectReferenceNotFound(reference_id))
}

fn read_reference_id(reader: &mut MemoryPackReader) -> Result<u32, MemoryPackError> {
    let value = varint::read_varint(reader)?;

    u32::try_from(value)
        .map_err(|_| MemoryPackError::InvalidLength(i32::try_from(value).unwrap_or(i32::MAX)))
}

fn read_lengths(reader: &mut MemoryPackReader, header: u8) -> Result<Vec<usize>, MemoryPackError> {
    if usize::from(header) > MAX_MEMBER_COUNT {
        return Err(MemoryPackError::InvalidLength(i32::from(header)));
    }

    (0..header)
        .map(|_| {
            usize::try_from(varint::read_varint(reader)?)
                .map_err(|_| MemoryPackError::UnexpectedEndOfBuffer)
        })
        .collect()
}
