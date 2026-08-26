mod collections;
mod multidim;
mod options;
mod primitives;
mod smart_ptrs;
mod special;
mod strings;
mod tuples;
mod unmanaged;

#[cfg(any(feature = "uuid", feature = "rust_decimal", feature = "half", feature = "num-bigint"))]
mod extended;

#[cfg(feature = "chrono")]
mod datetime;

#[cfg(any(feature = "glam", feature = "num-complex"))]
mod math;

#[allow(unused_imports)]
pub use {
    collections::*,
    multidim::*,
    options::*,
    primitives::*,
    smart_ptrs::*,
    special::*,
    strings::*,
    tuples::*
};

use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
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
            None => writer.write_u8(255)
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
        if marker == 255 {
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

pub(crate) use unmanaged::MemoryPackUnmanaged;

pub trait MemoryPackDeserializeZeroCopy<'a>: Sized {
    fn deserialize(reader: &mut MemoryPackReader<'a>) -> Result<Self, MemoryPackError>;
}
