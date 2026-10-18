use crate::{MemoryPackError, MemoryPackReader, MemoryPackWriter};

pub trait MemoryPackFormatter<T> {
    fn serialize(value: &T, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError>;

    fn deserialize(reader: &mut MemoryPackReader<'_>) -> Result<T, MemoryPackError>;

    fn serialized_size_hint(_value: &T) -> usize { 0 }
}
