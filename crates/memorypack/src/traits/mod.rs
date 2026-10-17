mod collections;
mod multidim;
mod options;
mod primitives;
mod serialization;
mod smart_ptrs;
mod special;
mod strings;
mod tuples;
mod unmanaged;

#[cfg(any(feature = "uuid", feature = "rust_decimal", feature = "half", feature = "num-bigint"))]
mod extended;

#[cfg(feature = "jiff")]
mod datetime;

#[cfg(any(feature = "glam", feature = "num-complex"))]
mod math;

pub use multidim::MultiDimArray;
pub use serialization::{
    MemoryPackDeserialize,
    MemoryPackDeserializeZeroCopy,
    MemoryPackSerialize
};
pub use special::{Lazy, Stack, Tuple, TupleElements};
pub(crate) use unmanaged::MemoryPackUnmanaged;
