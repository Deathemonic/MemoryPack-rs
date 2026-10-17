pub mod error;
pub mod traits;
pub mod prelude {
    #[cfg(feature = "derive")]
    pub use memorypack_derive::MemoryPackable;

    pub use crate::error::MemoryPackError;
    pub use crate::reader::MemoryPackReader;
    pub use crate::serializer::MemoryPackSerializer;
    pub use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
    pub use crate::writer::MemoryPackWriter;
}

mod graph;
mod io;
mod serialization;

pub use error::MemoryPackError;
pub use graph::{MemoryPackCircular, MemoryPackGraph, MemoryPackReference};
#[cfg(feature = "derive")]
pub use memorypack_derive::MemoryPackable;
pub use reader::MemoryPackReader;
pub use serialization::{serializer, serializer_options, state, varint};
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

use crate::io::{reader, writer};

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL_ALLOCATOR: mimalloc::MiMalloc = mimalloc::MiMalloc;
