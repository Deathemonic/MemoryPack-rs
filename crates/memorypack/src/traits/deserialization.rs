use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::serialization::format::NULL_OBJECT_HEADER;

pub trait MemoryPackDeserialize<'de>: Sized {
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError>;

    #[doc(hidden)]
    fn deserialize_nullable(
        reader: &mut MemoryPackReader<'de>
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
        reader: &mut MemoryPackReader<'de>,
        count: usize
    ) -> Result<Vec<Self>, MemoryPackError> {
        let mut values = Vec::with_capacity(count);

        for _ in 0..count {
            values.push(Self::deserialize(reader)?);
        }

        Ok(values)
    }
}
