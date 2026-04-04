use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;

/// Controls the representation of the nonlocal part of the pseudopotential.
///
/// Keyword type: String
///
/// Default: PspotNonlocalType::Reciprocal
///
/// Example:
/// PSPOT_NONLOCAL_TYPE : REAL
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PspotNonlocalType {
    /// Reciprocal space nonlocal pseudopotentials
    Reciprocal,
    /// Real space nonlocal pseudopotentials
    Real,
}

impl Default for PspotNonlocalType {
    fn default() -> Self {
        Self::Reciprocal
    }
}

impl FromCellValue for PspotNonlocalType {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "reciprocal" => Ok(Self::Reciprocal),
            "real" => Ok(Self::Real),
            other => Err(Error::Message(format!("unknown PspotNonlocalType: {other}"))),
        }
    }
}

impl FromKeyValue for PspotNonlocalType {
    const KEY_NAME: &'static str = "PSPOT_NONLOCAL_TYPE";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for PspotNonlocalType {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PSPOT_NONLOCAL_TYPE", self.to_cell_value())
    }
}

impl ToCellValue for PspotNonlocalType {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                PspotNonlocalType::Reciprocal => "RECIPROCAL",
                PspotNonlocalType::Real => "REAL",
            }
            .to_string(),
        )
    }
}

