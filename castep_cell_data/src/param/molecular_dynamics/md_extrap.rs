use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;

/// Determines the wavefunction extrapolation scheme used for MD.
///
/// Keyword type: String
///
/// Default: MdExtrap::First
///
/// Example:
/// MD_EXTRAP : Mixed
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MdExtrap {
    /// No extrapolation used
    None,
    /// First order extrapolation
    First,
    /// Second order extrapolation
    Second,
    /// Alternating first and second order extrapolation
    Mixed,
}

impl Default for MdExtrap {
    fn default() -> Self {
        Self::First // Default is First
    }
}

impl FromCellValue for MdExtrap {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "none" => Ok(Self::None),
            "first" => Ok(Self::First),
            "second" => Ok(Self::Second),
            "mixed" => Ok(Self::Mixed),
            other => Err(Error::Message(format!("unknown MdExtrap: {other}"))),
        }
    }
}

impl FromKeyValue for MdExtrap {
    const KEY_NAME: &'static str = "MD_EXTRAP";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MdExtrap {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_EXTRAP", self.to_cell_value())
    }
}

impl ToCellValue for MdExtrap {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                MdExtrap::None => "None",
                MdExtrap::First => "First",
                MdExtrap::Second => "Second",
                MdExtrap::Mixed => "Mixed",
            }
            .to_string(),
        )
    }
}


