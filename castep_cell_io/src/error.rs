use std::num::TryFromIntError;

use thiserror::Error;

pub type CResult<T> = std::result::Result<T, Error>;
#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Message(String),
    #[error("no entries left")]
    Empty,
    #[error("expected {0}, got {1}")]
    /// expected: {0}, got {1}
    UnexpectedType(String, String),
    #[error("key {0} not found")]
    /// key {0} not found
    KeyNotFound(String),
    #[error("{0}")]
    /// expected: {0}, from {1}
    TryFromInt(TryFromIntError),
}
