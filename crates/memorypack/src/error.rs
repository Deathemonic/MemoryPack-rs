use std::{io, string};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MemoryPackError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Utf8Error(#[from] string::FromUtf8Error),

    #[error("Invalid UTF-8 or UTF-16 string data")]
    InvalidUtf8,

    #[error("Invalid length: {0}")]
    InvalidLength(i32),

    #[error("Serialization error: {0}")]
    SerializationError(Box<str>),

    #[error("Deserialization error: {0}")]
    DeserializationError(Box<str>),

    #[error("Invalid URL: {0}")]
    #[cfg(feature = "url")]
    UrlParse(#[from] url::ParseError),

    #[error("Object is not found in this reference id: {0}")]
    ObjectReferenceNotFound(u32),

    #[error("Object is already added, id: {0}")]
    ObjectReferenceAlreadyAdded(u32),

    #[error("Object not found for update, id: {0}")]
    ObjectReferenceUpdateNotFound(u32),

    #[error("Invalid Lazy field count: expected 1, got {got}")]
    InvalidLazyFieldCount { got: u8 },

    #[error("Invalid Tuple field count: expected {expected}, got {got}")]
    InvalidTupleFieldCount { expected: u8, got: u8 },

    #[error("Unknown union tag {tag} for {name}")]
    UnknownUnionTag { tag: u8, name: &'static str },

    #[error("Invalid discriminant {value} for enum {name}")]
    InvalidEnumDiscriminant { value: i32, name: &'static str },

    #[error("DateTime out of range")]
    DateTimeOutOfRange,

    #[error("Duration out of range")]
    DurationOutOfRange,

    #[error("Date out of range")]
    DateOutOfRange,

    #[error("Invalid offset")]
    InvalidOffset,

    #[error("Invalid time ticks")]
    InvalidTimeTicks,

    #[error("Invalid date")]
    InvalidDate,

    #[error("Invalid array rank")]
    InvalidArrayRank,

    #[error("Negative length in BigInteger")]
    NegativeBigIntegerLength,

    #[error("Negative length in BigUint")]
    NegativeBigUintLength,

    #[error("Surrogate code unit cannot be converted to Rust char")]
    SurrogateCodeUnit,

    #[error("Buffer too small")]
    BufferTooSmall,

    #[error("Invalid Unicode code point")]
    InvalidCodePoint,

    #[error("Unexpected end of data")]
    UnexpectedEnd,

    #[error("Unexpected end of buffer")]
    UnexpectedEndOfBuffer,

    #[error("UTF-16 strings are not supported for zero-copy deserialization")]
    Utf16NotSupportedForZeroCopy
}
