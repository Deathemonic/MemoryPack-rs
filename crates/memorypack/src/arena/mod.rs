mod allocation;
mod collections;
mod deserialization;
mod options;
mod owned;
mod strings;

#[cfg(feature = "bumpalo")]
mod bumpalo;

pub use allocation::MemoryPackArena;
pub use deserialization::MemoryPackDeserializeIn;
