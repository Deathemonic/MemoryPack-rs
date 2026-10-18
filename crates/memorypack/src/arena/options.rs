use super::{MemoryPackArena, MemoryPackDeserializeIn};
use crate::{MemoryPackError, MemoryPackReader};

impl<'arena, Arena: MemoryPackArena + ?Sized, T: MemoryPackDeserializeIn<'arena, Arena>>
    MemoryPackDeserializeIn<'arena, Arena> for Option<T>
{
    #[inline]
    fn deserialize_in(
        reader: &mut MemoryPackReader<'_>,
        arena: &'arena Arena
    ) -> Result<Self, MemoryPackError> {
        T::deserialize_nullable_in(reader, arena)
    }
}
