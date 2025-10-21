use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;

/// MemoryPack serializer
pub struct MemoryPackSerializer;

impl MemoryPackSerializer {
    /// Serialize a value to a byte vector
    #[inline]
    pub fn serialize<T: MemoryPackSerialize>(value: &T) -> Result<Vec<u8>, MemoryPackError> {
        let mut writer = MemoryPackWriter::new();
        value.serialize(&mut writer)?;
        Ok(writer.into_bytes())
    }

    /// Serialize a value to an existing writer
    #[inline]
    pub fn serialize_to<T: MemoryPackSerialize>(
        value: &T,
        writer: &mut MemoryPackWriter,
    ) -> Result<(), MemoryPackError> {
        value.serialize(writer)
    }

    /// Deserialize a value from bytes
    #[inline]
    pub fn deserialize<T: MemoryPackDeserialize>(data: &[u8]) -> Result<T, MemoryPackError> {
        let mut reader = MemoryPackReader::new(data);
        T::deserialize(&mut reader)
    }

    /// Deserialize a value from an existing reader
    #[inline]
    pub fn deserialize_from<T: MemoryPackDeserialize>(reader: &mut MemoryPackReader) -> Result<T, MemoryPackError> {
        T::deserialize(reader)
    }
}