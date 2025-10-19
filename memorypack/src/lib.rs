//! High-performance binary serialization for Rust, inspired by C# MemoryPack.
//!
//! # Quick Start
//!
//! ```rust
//! use memorypack::prelude::*;
//!
//! #[derive(MemoryPackable)]
//! struct Person {
//!     age: i32,
//!     name: String,
//! }
//!
//! let person = Person { age: 40, name: "John".to_string() };
//! let bytes = MemoryPackSerializer::serialize(&person)?;
//! let decoded: Person = MemoryPackSerializer::deserialize(&bytes)?;
//! # Ok::<(), memorypack::MemoryPackError>(())
//! ```
//!
//! # Enums
//!
//! C-like enums are serialized as i32:
//!
//! ```rust
//! use memorypack::prelude::*;
//!
//! #[derive(MemoryPackable)]
//! #[repr(i32)]
//! enum Status {
//!     Pending = 0,
//!     Active = 1,
//!     Completed = 2,
//! }
//! # Ok::<(), memorypack::MemoryPackError>(())
//! ```
//!
//! # Flags / Bitfields
//!
//! Use `#[repr(transparent)]` for flag-style enums:
//!
//! ```rust
//! use memorypack::prelude::*;
//!
//! #[derive(MemoryPackable)]
//! #[repr(transparent)]
//! struct Permissions(i32);
//!
//! impl Permissions {
//!     const READ: Self = Self(1);
//!     const WRITE: Self = Self(2);
//!     const EXECUTE: Self = Self(4);
//! }
//!
//! impl std::ops::BitOr for Permissions {
//!     type Output = Self;
//!     fn bitor(self, rhs: Self) -> Self {
//!         Self(self.0 | rhs.0)
//!     }
//! }
//!
//! let perms = Permissions::READ | Permissions::WRITE;
//! let bytes = MemoryPackSerializer::serialize(&perms)?;
//! # Ok::<(), memorypack::MemoryPackError>(())
//! ```

mod collections;
mod reader;
mod writer;

pub mod error;
pub mod serializer;
pub mod traits;

pub use error::{MemoryPackError, Result};
pub use reader::MemoryPackReader;
pub use serializer::MemoryPackSerializer;
pub use traits::{MemoryPackDeserialize, MemoryPackSerialize};
pub use writer::MemoryPackWriter;

#[cfg(feature = "derive")]
pub use memorypack_derive::MemoryPackable;

pub mod prelude {
    pub use crate::error::{MemoryPackError, Result};
    pub use crate::serializer::MemoryPackSerializer;
    pub use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};

    #[cfg(feature = "derive")]
    pub use memorypack_derive::MemoryPackable;
}
