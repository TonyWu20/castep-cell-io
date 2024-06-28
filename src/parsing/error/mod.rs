use std::fmt::Display;

#[derive(Debug)]
pub enum CellParseError {
    FileReadingFailure,
    UnexpectedLength,
    UnexpectedValueType,
    Invalid,
    GetBlockDataFailure,
    GetFieldDataFailure,
    RequiredSectionMissing,
}

impl Display for CellParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedLength => f.write_str("Unexpected length of data"),
            Self::UnexpectedValueType => f.write_str("Unexpected value type"),
            Self::Invalid => f.write_str("Not valid keywords or data in `.cell`"),
            Self::GetBlockDataFailure => f.write_str("Fail to get block data"),
            Self::GetFieldDataFailure => f.write_str("Fail to get field data"),
            Self::RequiredSectionMissing => {
                f.write_str("Missing lattice parameters and/or ionic positions!")
            }
            Self::FileReadingFailure => f.write_str("Failed to read from file"),
        }
    }
}

impl std::error::Error for CellParseError {}
