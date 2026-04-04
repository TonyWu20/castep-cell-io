use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;

/// Controls the representation of the nonlocal part of the pseudopotential
/// used for calculation of the <β|ϕ> overlaps.
///
/// Keyword type: String (expert)
///
/// Default: The value of PSPOT_NONLOCAL_TYPE (handled by parent struct/context).
///
/// Example:
/// PSPOT_BETA_PHI_TYPE : REAL
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PspotBetaPhiType {
    /// Reciprocal space nonlocal pseudopotentials
    Reciprocal,
    /// Real space nonlocal pseudopotentials
    Real,
}

impl FromCellValue for PspotBetaPhiType {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "reciprocal" => Ok(Self::Reciprocal),
            "real" => Ok(Self::Real),
            other => Err(Error::Message(format!("unknown PspotBetaPhiType: {other}"))),
        }
    }
}

impl FromKeyValue for PspotBetaPhiType {
    const KEY_NAME: &'static str = "PSPOT_BETA_PHI_TYPE";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for PspotBetaPhiType {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PSPOT_BETA_PHI_TYPE", self.to_cell_value())
    }
}

impl ToCellValue for PspotBetaPhiType {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                PspotBetaPhiType::Reciprocal => "RECIPROCAL",
                PspotBetaPhiType::Real => "REAL",
            }
            .to_string(),
        )
    }
}

