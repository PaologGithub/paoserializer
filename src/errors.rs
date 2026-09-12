use core::fmt::{self, Display};

#[derive(Debug)]
pub enum DeserializeError {
    InvalidLength,
    InvalidMagic,
    InvalidHash,
    ZStdError(zstd_safe::ErrorCode),
    PostCardError(postcard::Error),
}

impl Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeserializeError::InvalidLength => write!(f, "Invalid bytes length"),
            DeserializeError::InvalidMagic => write!(f, "Invalid magic"),
            DeserializeError::InvalidHash => write!(f, "Invalid hash"),
            DeserializeError::ZStdError(error) => write!(f, "zstd Error: {}", error),
            DeserializeError::PostCardError(error) => write!(f, "postcard Error: {}", error),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DeserializeError {}

#[derive(Debug)]
pub enum SerializeError {
    PostCardError(postcard::Error),
    ZStdError(zstd_safe::ErrorCode),
}

impl Display for SerializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SerializeError::ZStdError(error) => write!(f, "zstd Error: {}", error),
            SerializeError::PostCardError(error) => write!(f, "postcard Error: {}", error),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SerializeError {}
