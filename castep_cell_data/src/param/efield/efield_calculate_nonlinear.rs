use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;

/// Specifies which non-linear optical property to calculate during a TASK=EFIELD calculation.
///
/// Keyword type: String
///
/// Default: EfieldCalculateNonlinear::None
///
/// Example:
/// EFIELD_CALCULATE_NONLINEAR : CHI2
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EfieldCalculateNonlinear {
    /// Produces second harmonic generation coefficients
    Chi2,
    /// Non-linear optical properties are not calculated
    None,
}

impl Default for EfieldCalculateNonlinear {
    fn default() -> Self {
        Self::None
    }
}

impl FromCellValue for EfieldCalculateNonlinear {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "chi2" => Ok(Self::Chi2),
            "none" => Ok(Self::None),
            other => Err(Error::Message(format!("unknown EfieldCalculateNonlinear: {other}"))),
        }
    }
}

impl FromKeyValue for EfieldCalculateNonlinear {
    const KEY_NAME: &'static str = "EFIELD_CALCULATE_NONLINEAR";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for EfieldCalculateNonlinear {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("EFIELD_CALCULATE_NONLINEAR", self.to_cell_value())
    }
}

impl ToCellValue for EfieldCalculateNonlinear {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                EfieldCalculateNonlinear::Chi2 => "CHI2",
                EfieldCalculateNonlinear::None => "NONE",
            }
            .to_string(),
        )
    }
}

