use crate::arena::{MemoryPackArena, MemoryPackDeserializeIn};
use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::serializer_options::MemoryPackSerializerOptions;
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;

const DEFAULT_BUFFER_CAPACITY: usize = 64;

pub struct MemoryPackSerializer;

impl MemoryPackSerializer {
    #[inline]
    pub fn serialize<T: MemoryPackSerialize>(value: &T) -> Result<Vec<u8>, MemoryPackError> {
        Self::serialize_with_options(value, &MemoryPackSerializerOptions::UTF8)
    }

    #[inline]
    pub fn serialize_with_options<T: MemoryPackSerialize>(
        value: &T,
        options: &MemoryPackSerializerOptions
    ) -> Result<Vec<u8>, MemoryPackError> {
        let mut writer = MemoryPackWriter::with_capacity_and_options(
            value
                .serialized_size_hint()
                .max(DEFAULT_BUFFER_CAPACITY),
            *options
        );

        value.serialize(&mut writer)?;
        Ok(writer.into_bytes())
    }

    #[inline]
    pub fn serialize_to<T: MemoryPackSerialize>(
        value: &T,
        writer: &mut MemoryPackWriter
    ) -> Result<(), MemoryPackError> {
        value.serialize(writer)
    }

    #[inline]
    pub fn deserialize<'de, T: MemoryPackDeserialize<'de>>(
        data: &'de [u8]
    ) -> Result<T, MemoryPackError> {
        let mut reader = MemoryPackReader::new(data);

        T::deserialize(&mut reader)
    }

    #[inline]
    pub fn deserialize_from<'de, T: MemoryPackDeserialize<'de>>(
        reader: &mut MemoryPackReader<'de>
    ) -> Result<T, MemoryPackError> {
        T::deserialize(reader)
    }

    #[inline]
    pub fn deserialize_in<
        'arena,
        T: MemoryPackDeserializeIn<'arena, Arena>,
        Arena: MemoryPackArena + ?Sized
    >(
        data: &[u8],
        arena: &'arena Arena
    ) -> Result<T, MemoryPackError> {
        let mut reader = MemoryPackReader::new(data);

        T::deserialize_in(&mut reader, arena)
    }
}
