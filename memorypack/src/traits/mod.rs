mod collections;
mod multidim;
mod options;
mod primitives;
mod smart_ptrs;
mod strings;
mod tuples;

#[cfg(any(
    feature = "uuid",
    feature = "rust_decimal",
    feature = "half",
    feature = "num-bigint"
))]
mod extended;

#[cfg(feature = "chrono")]
mod datetime;

#[allow(unused_imports)]
pub use {
    collections::*, multidim::*, options::*, primitives::*, smart_ptrs::*, strings::*, tuples::*,
};

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
