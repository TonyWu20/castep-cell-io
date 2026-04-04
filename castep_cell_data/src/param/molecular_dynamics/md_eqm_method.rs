use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;

/// Determines the scheme to be used for enhanced MD equilibration.
///
/// Keyword type: String
///
/// Default: MdEqmMethod::None
///
/// Example:
/// MD_EQM_METHOD : BERENDSEN
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MdEqmMethod {
    /// No enhanced equilibration
    None,
    /// Berendsen weak coupling scheme
    Berendsen,
}

impl Default for MdEqmMethod {
    fn default() -> Self {
        Self::None // Default is NONE
    }
}

impl FromCellValue for MdEqmMethod {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "none" => Ok(Self::None),
            "berendsen" => Ok(Self::Berendsen),
            other => Err(Error::Message(format!("unknown MdEqmMethod: {other}"))),
        }
    }
}

impl FromKeyValue for MdEqmMethod {
    const KEY_NAME: &'static str = "MD_EQM_METHOD";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MdEqmMethod {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_EQM_METHOD", self.to_cell_value())
    }
}

impl ToCellValue for MdEqmMethod {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                MdEqmMethod::None => "NONE",
                MdEqmMethod::Berendsen => "BERENDSEN",
            }
            .to_string(),
        )
    }
}


