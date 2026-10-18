use crate::MemoryPackError;

pub trait MemoryPackArena {
    fn allocate<T>(&self, value: T) -> &T;

    fn allocate_str(&self, value: &str) -> &str;

    fn try_allocate_string(
        &self,
        capacity: usize,
        characters: impl IntoIterator<Item = Result<char, MemoryPackError>>
    ) -> Result<&str, MemoryPackError>;

    fn try_allocate_slice<'arena, T: 'arena>(
        &'arena self,
        capacity: usize,
        elements: impl IntoIterator<Item = Result<T, MemoryPackError>>
    ) -> Result<&'arena [T], MemoryPackError>;
}
