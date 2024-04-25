use std::fmt::Display;

use serde::de;

#[derive(Debug)]
#[non_exhaustive]
pub enum DeError {
    Message(String),
    Eof,
    Syntax,
    ExpectedInteger,
    ExpectedFloat,
}

impl Display for DeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Deserialize Error!")
    }
}

impl de::Error for DeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        DeError::Message(msg.to_string())
    }
}

impl std::error::Error for DeError {}
