mod reader;
mod writer;

pub mod error;
pub mod serializer;
pub mod serializer_options;
pub mod state;
pub mod traits;
pub mod varint;

pub use error::MemoryPackError;
#[cfg(feature = "derive")]
pub use memorypack_derive::MemoryPackable;
pub use reader::MemoryPackReader;
pub use serializer::MemoryPackSerializer;
pub use serializer_options::{MemoryPackSerializerOptions, StringEncoding};
pub use state::{MemoryPackReaderOptionalState, MemoryPackWriterOptionalState};
pub use traits::{
    Lazy,
    MemoryPackDeserialize,
    MemoryPackDeserializeZeroCopy,
    MemoryPackSerialize,
    MultiDimArray,
    Stack,
    Tuple
};
pub use writer::MemoryPackWriter;

pub mod prelude {
    #[cfg(feature = "derive")]
    pub use memorypack_derive::MemoryPackable;

    pub use crate::error::MemoryPackError;
    pub use crate::reader::MemoryPackReader;
    pub use crate::serializer::MemoryPackSerializer;
    pub use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
    pub use crate::writer::MemoryPackWriter;
}
