use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::serialization::format::{
    COLLECTION_HEADER_SIZE,
    NULL_COLLECTION_LENGTH,
    UTF8_STRING_HEADER_SIZE
};
use crate::traits::{MemoryPackDeserialize, MemoryPackDeserializeZeroCopy, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;

impl MemoryPackSerialize for String {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_string(self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize {
        if self.is_empty() {
            COLLECTION_HEADER_SIZE
        } else {
            UTF8_STRING_HEADER_SIZE + self.len()
        }
    }

    fn serialize_nullable(
        value: Option<&Self>,
        writer: &mut MemoryPackWriter
    ) -> Result<(), MemoryPackError> {
        writer.write_string_option(value.map(Self::as_str))
    }

    fn nullable_size_hint(value: Option<&Self>) -> usize {
        value.map_or(COLLECTION_HEADER_SIZE, MemoryPackSerialize::serialized_size_hint)
    }
}

impl MemoryPackDeserialize for String {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_string()
    }

    fn deserialize_nullable(
        reader: &mut MemoryPackReader
    ) -> Result<Option<Self>, MemoryPackError> {
        let marker = reader.read_i32()?;

        if marker == NULL_COLLECTION_LENGTH {
            Ok(None)
        } else {
            reader.rewind(COLLECTION_HEADER_SIZE)?;
            Ok(Some(Self::deserialize(reader)?))
        }
    }
}

impl MemoryPackSerialize for &str {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_string(self)
    }

    #[inline(always)]
    fn serialized_size_hint(&self) -> usize {
        if self.is_empty() {
            COLLECTION_HEADER_SIZE
        } else {
            UTF8_STRING_HEADER_SIZE + self.len()
        }
    }
}

impl<'a> MemoryPackDeserializeZeroCopy<'a> for &'a str {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'a>) -> Result<Self, MemoryPackError> {
        reader.read_str()
    }
}
