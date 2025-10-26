use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::traits::{MemoryPackDeserialize, MemoryPackDeserializeZeroCopy, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;

impl MemoryPackSerialize for String {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_string(self)
    }
}

impl MemoryPackDeserialize for String {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        reader.read_string()
    }
}

impl MemoryPackSerialize for &str {
    #[inline(always)]
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        writer.write_string(self)
    }
}

impl<'a> MemoryPackDeserializeZeroCopy<'a> for &'a str {
    #[inline(always)]
    fn deserialize(reader: &mut MemoryPackReader<'a>) -> Result<Self, MemoryPackError> {
        reader.read_str()
    }
}
