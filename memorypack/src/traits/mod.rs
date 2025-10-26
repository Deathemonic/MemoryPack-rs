mod primitives;
mod strings;
mod collections;
mod options;
mod smart_ptrs;
mod tuples;
mod multidim;

#[allow(unused_imports)]
pub use {primitives::*, strings::*, collections::*, options::*, smart_ptrs::*, tuples::*, multidim::*};

use crate::error::MemoryPackError;
use crate::reader::MemoryPackReader;
use crate::writer::MemoryPackWriter;

pub trait MemoryPackSerialize {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError>;
}

pub trait MemoryPackDeserialize: Sized {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError>;
}

pub trait MemoryPackDeserializeZeroCopy<'a>: Sized {
    fn deserialize(reader: &mut MemoryPackReader<'a>) -> Result<Self, MemoryPackError>;
}
