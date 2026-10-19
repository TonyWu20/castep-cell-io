//! Errors for reading and writing CASTEP binary model files.

use thiserror::Error;

/// A version number parsed from a model file. `major.minor`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileVersion {
    pub major: u8,
    pub minor: u8,
}

impl FileVersion {
    pub const fn new(major: u8, minor: u8) -> Self {
        Self { major, minor }
    }
}

impl std::fmt::Display for FileVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

/// All fallible operations on model files return [`CheckError`].
#[derive(Debug, Error)]
pub enum CheckError {
    #[error("i/o error on {path}: {source}")]
    Io {
        path: std::path::PathBuf,
        source: std::io::Error,
    },

    #[error("i/o error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("truncated record at offset {offset}")]
    Truncated { offset: u64 },

    #[error("record marker mismatch at offset {offset}: header {header}, trailer {trailer}")]
    MarkerMismatch { offset: u64, header: u32, trailer: u32 },

    #[error("record at offset {offset} is {actual} bytes, expected {expected}")]
    BadRecordSize { offset: u64, expected: usize, actual: usize },

    #[error("record payload length {0} is not a multiple of the element size")]
    BadPayloadLength(usize),

    #[error("logical record value {0:#010x} is not 0 or 1")]
    BadLogical(u32),

    #[error("malformed version record {0:?}")]
    MalformedVersion(Vec<u8>),

    #[error("file version {file} is newer than target {target}; a later release feature is required")]
    VersionTooNew { file: FileVersion, target: FileVersion },

    #[error("file version {0} is older than the supported floor 3.0")]
    VersionTooOld(FileVersion),

    #[error("unknown optional section tag {0:?}")]
    UnknownSection(String),

    #[error("section error: {0}")]
    Section(String),
}

/// Result type used throughout the crate.
pub type CResult<T> = std::result::Result<T, CheckError>;
