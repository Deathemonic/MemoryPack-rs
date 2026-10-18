use bumpalo::Bump;
use bumpalo::collections::{String as ArenaString, Vec as ArenaVec};

use super::MemoryPackArena;
use crate::MemoryPackError;

impl MemoryPackArena for Bump {
    #[inline]
    fn allocate<T>(&self, value: T) -> &T { self.alloc(value) }

    #[inline]
    fn allocate_str(&self, value: &str) -> &str { self.alloc_str(value) }

    #[inline]
    fn try_allocate_string(
        &self,
        capacity: usize,
        characters: impl IntoIterator<Item = Result<char, MemoryPackError>>
    ) -> Result<&str, MemoryPackError> {
        let mut value = ArenaString::with_capacity_in(capacity, self);

        for character in characters {
            value.push(character?);
        }

        Ok(value.into_bump_str())
    }

    #[inline]
    fn try_allocate_slice<'arena, T: 'arena>(
        &'arena self,
        capacity: usize,
        elements: impl IntoIterator<Item = Result<T, MemoryPackError>>
    ) -> Result<&'arena [T], MemoryPackError> {
        let mut values = ArenaVec::with_capacity_in(capacity, self);

        for element in elements {
            values.push(element?);
        }

        Ok(values.into_bump_slice())
    }
}
