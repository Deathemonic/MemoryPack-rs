use super::MemoryPackArena;
use crate::serialization::format::NULL_OBJECT_HEADER;
use crate::{MemoryPackError, MemoryPackReader};

pub trait MemoryPackDeserializeIn<'arena, Arena: MemoryPackArena + ?Sized>: Sized {
    fn deserialize_in(
        reader: &mut MemoryPackReader<'_>,
        arena: &'arena Arena
    ) -> Result<Self, MemoryPackError>;

    fn deserialize_nullable_in(
        reader: &mut MemoryPackReader<'_>,
        arena: &'arena Arena
    ) -> Result<Option<Self>, MemoryPackError> {
        let marker = reader.read_u8()?;

        if marker == NULL_OBJECT_HEADER {
            return Ok(None);
        }

        reader.rewind(size_of::<u8>())?;

        Self::deserialize_in(reader, arena).map(Some)
    }
}
