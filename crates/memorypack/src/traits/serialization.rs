use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::serialization::format::NULL_OBJECT_HEADER;
use crate::writer::MemoryPackWriter;

pub trait MemoryPackSerialize {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError>;

    #[doc(hidden)]
    fn serialize_nullable(
        value: Option<&Self>,
        writer: &mut MemoryPackWriter
    ) -> Result<(), MemoryPackError>
    where
        Self: Sized
    {
        match value {
            Some(value) => value.serialize(writer),
            None => writer.write_u8(NULL_OBJECT_HEADER)
        }
    }

    #[doc(hidden)]
    fn nullable_size_hint(value: Option<&Self>) -> usize
    where
        Self: Sized
    {
        value.map_or(1, MemoryPackSerialize::serialized_size_hint)
    }

    #[doc(hidden)]
    #[inline]
    fn serialize_many(values: &[Self], writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError>
    where
        Self: Sized
    {
        for value in values {
            value.serialize(writer)?;
        }

        Ok(())
    }

    #[inline]
    fn serialized_size_hint(&self) -> usize { 0 }
}

pub trait MemoryPackDeserialize: Sized {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError>;

    #[doc(hidden)]
    fn deserialize_nullable(
        reader: &mut MemoryPackReader
    ) -> Result<Option<Self>, MemoryPackError> {
        let marker = reader.read_u8()?;

        if marker == NULL_OBJECT_HEADER {
            Ok(None)
        } else {
            reader.rewind(1)?;
            Ok(Some(Self::deserialize(reader)?))
        }
    }

    #[doc(hidden)]
    #[inline]
    fn deserialize_many(
        reader: &mut MemoryPackReader,
        count: usize
    ) -> Result<Vec<Self>, MemoryPackError> {
        let mut values = Vec::with_capacity(count);

        for _ in 0..count {
            values.push(Self::deserialize(reader)?);
        }

        Ok(values)
    }
}

pub trait MemoryPackDeserializeZeroCopy<'a>: Sized {
    fn deserialize(reader: &mut MemoryPackReader<'a>) -> Result<Self, MemoryPackError>;
}
