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
    KeyNotFound(String),
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::Message(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::Message(msg.to_string())
    }
}
