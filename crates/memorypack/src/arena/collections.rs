use std::mem::needs_drop;
use std::rc::Rc;
use std::sync::Arc;

use super::{MemoryPackArena, MemoryPackDeserializeIn};
use crate::serialization::format::{COLLECTION_HEADER_SIZE, NULL_COLLECTION_LENGTH};
use crate::{MemoryPackError, MemoryPackReader};

const EMPTY_COLLECTION_LENGTH: i32 = 0;

impl<
    'arena: 'value,
    'value,
    Arena: MemoryPackArena + ?Sized,
    T: MemoryPackDeserializeIn<'arena, Arena>
> MemoryPackDeserializeIn<'arena, Arena> for &'value [T]
{
    #[inline]
    fn deserialize_in(
        reader: &mut MemoryPackReader<'_>,
        arena: &'arena Arena
    ) -> Result<Self, MemoryPackError> {
        validate_arena_value::<T>()?;
        let length = read_collection_length(reader)?;

        arena.try_allocate_slice(length, (0..length).map(|_| T::deserialize_in(reader, arena)))
    }

    #[inline]
    fn deserialize_nullable_in(
        reader: &mut MemoryPackReader<'_>,
        arena: &'arena Arena
    ) -> Result<Option<Self>, MemoryPackError> {
        let marker = reader.read_i32()?;

        if marker == NULL_COLLECTION_LENGTH {
            return Ok(None);
        }

        reader.rewind(COLLECTION_HEADER_SIZE)?;

        Self::deserialize_in(reader, arena).map(Some)
    }
}

impl<
    'arena: 'value,
    'value,
    Arena: MemoryPackArena + ?Sized,
    T: MemoryPackDeserializeIn<'arena, Arena>
> MemoryPackDeserializeIn<'arena, Arena> for &'value T
{
    #[inline]
    fn deserialize_in(
        reader: &mut MemoryPackReader<'_>,
        arena: &'arena Arena
    ) -> Result<Self, MemoryPackError> {
        validate_arena_value::<T>()?;
        let value = T::deserialize_in(reader, arena)?;

        Ok(arena.allocate(value))
    }

    #[inline]
    fn deserialize_nullable_in(
        reader: &mut MemoryPackReader<'_>,
        arena: &'arena Arena
    ) -> Result<Option<Self>, MemoryPackError> {
        validate_arena_value::<T>()?;
        let value = T::deserialize_nullable_in(reader, arena)?;

        Ok(value.map(|value| arena.allocate(value)))
    }
}

impl<'arena, Arena: MemoryPackArena + ?Sized, T: MemoryPackDeserializeIn<'arena, Arena>>
    MemoryPackDeserializeIn<'arena, Arena> for Vec<T>
{
    #[inline]
    fn deserialize_in(
        reader: &mut MemoryPackReader<'_>,
        arena: &'arena Arena
    ) -> Result<Self, MemoryPackError> {
        let length = read_collection_length(reader)?;
        let mut values = Self::with_capacity(length);

        for _ in 0..length {
            values.push(T::deserialize_in(reader, arena)?);
        }

        Ok(values)
    }

    #[inline]
    fn deserialize_nullable_in(
        reader: &mut MemoryPackReader<'_>,
        arena: &'arena Arena
    ) -> Result<Option<Self>, MemoryPackError> {
        let marker = reader.read_i32()?;

        if marker == NULL_COLLECTION_LENGTH {
            return Ok(None);
        }

        reader.rewind(COLLECTION_HEADER_SIZE)?;

        Self::deserialize_in(reader, arena).map(Some)
    }
}

macro_rules! deserialize_pointer_in {
    ($($pointer:ident),+ $(,)?) => {
        $(
            impl<'arena, Arena, T> MemoryPackDeserializeIn<'arena, Arena> for $pointer<T>
            where
                Arena: MemoryPackArena + ?Sized,
                T: MemoryPackDeserializeIn<'arena, Arena>
            {
                #[inline]
                fn deserialize_in(
                    reader: &mut MemoryPackReader<'_>,
                    arena: &'arena Arena
                ) -> Result<Self, MemoryPackError> {
                    Ok($pointer::new(T::deserialize_in(reader, arena)?))
                }
            }
        )+
    };
}

deserialize_pointer_in!(Box, Rc, Arc);

pub fn read_collection_length(reader: &mut MemoryPackReader<'_>) -> Result<usize, MemoryPackError> {
    let length = reader.read_i32()?;

    match length {
        NULL_COLLECTION_LENGTH | EMPTY_COLLECTION_LENGTH => Ok(0),
        length if length < 0 => Err(MemoryPackError::InvalidLength(length)),
        length => Ok(length as usize)
    }
}

const fn validate_arena_value<T>() -> Result<(), MemoryPackError> {
    if needs_drop::<T>() {
        return Err(MemoryPackError::ArenaValueRequiresDrop);
    }

    Ok(())
}
