use simdutf8::basic;

use super::{MemoryPackArena, MemoryPackDeserializeIn};
use crate::serialization::format::{COLLECTION_HEADER_SIZE, NULL_COLLECTION_LENGTH};
use crate::{MemoryPackError, MemoryPackReader};

const NULL_STRING_MARKER: i32 = -1;
const EMPTY_STRING_MARKER: i32 = 0;
const MAX_UTF8_BYTES_PER_UTF16_UNIT: usize = 3;

impl<'arena: 'value, 'value, Arena: MemoryPackArena + ?Sized> MemoryPackDeserializeIn<'arena, Arena>
    for &'value str
{
    #[inline]
    fn deserialize_in(
        reader: &mut MemoryPackReader<'_>,
        arena: &'arena Arena
    ) -> Result<Self, MemoryPackError> {
        let marker = reader.read_i32()?;

        match marker {
            NULL_STRING_MARKER | EMPTY_STRING_MARKER => Ok(""),
            length if length < 0 => read_utf8_in(reader, (!length) as usize, arena),
            length => read_utf16_in(reader, length, arena)
        }
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

fn read_utf8_in<'arena, Arena: MemoryPackArena + ?Sized>(
    reader: &mut MemoryPackReader<'_>,
    byte_count: usize,
    arena: &'arena Arena
) -> Result<&'arena str, MemoryPackError> {
    reader.read_i32()?;
    let bytes = reader.read_bytes(byte_count)?;
    let value = basic::from_utf8(bytes).map_err(|_| MemoryPackError::InvalidUtf8)?;

    Ok(arena.allocate_str(value))
}

fn read_utf16_in<'arena, Arena: MemoryPackArena + ?Sized>(
    reader: &mut MemoryPackReader<'_>,
    character_count: i32,
    arena: &'arena Arena
) -> Result<&'arena str, MemoryPackError> {
    let count = character_count as usize;
    let bytes = reader.read_bytes(count * size_of::<u16>())?;
    let capacity = count
        .checked_mul(MAX_UTF8_BYTES_PER_UTF16_UNIT)
        .ok_or(MemoryPackError::InvalidLength(character_count))?;
    let units = bytes
        .as_chunks::<{ size_of::<u16>() }>()
        .0
        .iter()
        .copied()
        .map(u16::from_le_bytes);

    let characters = char::decode_utf16(units)
        .map(|character| character.map_err(|_| MemoryPackError::InvalidUtf8));

    arena.try_allocate_string(capacity, characters)
}
