use thiserror::Error;

#[derive(Debug, Error)]
pub enum MemoryPackError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("Invalid UTF-8 or UTF-16 string data")]
    InvalidUtf8,

    #[error("Invalid length: {0}")]
    InvalidLength(i32),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("Buffer too small")]
    BufferTooSmall,

    #[error("Invalid Unicode code point")]
    InvalidCodePoint,

    #[error("Unexpected end of data")]
    UnexpectedEnd,

    #[error("Unexpected end of buffer")]
    UnexpectedEndOfBuffer,

    #[error("UTF-16 strings are not supported for zero-copy deserialization")]
    Utf16NotSupportedForZeroCopy,
}
