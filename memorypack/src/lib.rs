mod collections;
mod reader;
mod writer;

pub mod varint;
pub mod error;
pub mod serializer;
pub mod traits;

pub use error::MemoryPackError;
pub use reader::MemoryPackReader;
pub use serializer::MemoryPackSerializer;
pub use traits::{MemoryPackDeserialize, MemoryPackSerialize};
pub use writer::MemoryPackWriter;

#[cfg(feature = "derive")]
pub use memorypack_derive::MemoryPackable;

pub mod prelude {
    pub use crate::error::MemoryPackError;
    pub use crate::serializer::MemoryPackSerializer;
    pub use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};

    #[cfg(feature = "derive")]
    pub use memorypack_derive::MemoryPackable;
}
