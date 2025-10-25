#![cfg_attr(feature = "nightly", feature(specialization))]
#![cfg_attr(feature = "nightly", allow(incomplete_features))]

mod collections;
mod reader;
mod writer;

pub mod varint;
pub mod error;
pub mod serializer;
pub mod state;
pub mod traits;

pub use error::MemoryPackError;
pub use reader::MemoryPackReader;
pub use serializer::MemoryPackSerializer;
pub use state::{MemoryPackReaderOptionalState, MemoryPackWriterOptionalState};
pub use traits::{MemoryPackDeserialize, MemoryPackDeserializeZeroCopy, MemoryPackSerialize};
pub use writer::MemoryPackWriter;

#[cfg(not(feature = "nightly"))]
pub use traits::{NullableString, NullableVec};

#[cfg(feature = "derive")]
pub use memorypack_derive::MemoryPackable;

pub mod prelude {
    pub use crate::error::MemoryPackError;
    pub use crate::reader::MemoryPackReader;
    pub use crate::serializer::MemoryPackSerializer;
    pub use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
    pub use crate::writer::MemoryPackWriter;

    #[cfg(feature = "derive")]
    pub use memorypack_derive::MemoryPackable;
}
