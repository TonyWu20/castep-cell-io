use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_bool;
use serde::{Deserialize, Serialize};

/// Specifies whether to calculate the non-analytical contribution for LO/TO splitting.
///
/// Keyword type: Logical
///
/// Default: true
///
/// Example:
/// PHONON_CALC_LO_TO_SPLITTING : FALSE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "PHONON_CALC_LO_TO_SPLITTING")]
pub struct PhononCalcLoToSplitting(pub bool);

impl FromCellValue for PhononCalcLoToSplitting {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl FromKeyValue for PhononCalcLoToSplitting {
    const KEY_NAME: &'static str = "PHONON_CALC_LO_TO_SPLITTING";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl Default for PhononCalcLoToSplitting {
    fn default() -> Self {
        Self(true) // Default is TRUE
    }
}

impl ToCell for PhononCalcLoToSplitting {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PHONON_CALC_LO_TO_SPLITTING", CellValue::Bool(self.0))
    }
}

impl ToCellValue for PhononCalcLoToSplitting {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}


