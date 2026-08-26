use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;

impl<T: MemoryPackSerialize> MemoryPackSerialize for Option<T> {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        T::serialize_nullable(self.as_ref(), writer)
    }

    fn serialized_size_hint(&self) -> usize { T::nullable_size_hint(self.as_ref()) }
}

impl<T: MemoryPackDeserialize> MemoryPackDeserialize for Option<T> {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        T::deserialize_nullable(reader)
    }
}
