use std::fmt::Display;

#[derive(Debug)]
pub enum CellParseError {
    UnexpectedLength,
    UnexpectedValueType,
    Invalid,
    GetBlockDataFailure,
    RequiredSectionMissing,
}

impl Display for CellParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedLength => f.write_str("Unexpected length of data"),
            Self::UnexpectedValueType => f.write_str("Unexpected value type"),
            Self::Invalid => f.write_str("Not valid keywords or data in `.cell`"),
            Self::GetBlockDataFailure => f.write_str("Fail to get block data"),
            Self::RequiredSectionMissing => {
                f.write_str("Missing lattice parameters and/or ionic positions!")
            }
        }
    }
}

impl std::error::Error for CellParseError {}
