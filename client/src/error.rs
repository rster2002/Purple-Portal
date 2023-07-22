
use std::string::FromUtf8Error;
use diamond_types::list::encoding::encode_tools::ParseError;
use thiserror::Error;
use crate::traits::fs_adapter::{FsAdapter, FsAdapterError};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Client IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Bincode error: {0}")]
    Bincode(#[from] Box<bincode::ErrorKind>),

    #[error("FS Adapter error: {0}")]
    FsAdapterError(String),

    #[error("Failed to parse oplog: {0}")]
    OpLogParseError(#[from] ParseError),

    #[error("UTF-8 encoding error: {0}")]
    Utf8EncodingError(#[from] FromUtf8Error),
}

impl<T> From<T> for Error
    where T: FsAdapterError,
{
    fn from(value: T) -> Self {
        Error::FsAdapterError(value.to_string())
    }
}
