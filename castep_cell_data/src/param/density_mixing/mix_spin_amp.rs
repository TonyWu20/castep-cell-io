use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult};
use castep_cell_io::query::value_as_f64;
use serde::{Deserialize, Serialize};

/// Determines the mixing amplitude for the spin density in the density mixing procedure.
///
/// Keyword type: Real
///
/// Default: 2.0
///
/// Example:
/// MIX_SPIN_AMP : 1.754
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "MIX_SPIN_AMP")]
pub struct MixSpinAmp(pub f64);

impl Default for MixSpinAmp {
    fn default() -> Self {
        Self(2.0)
    }
}

impl FromCellValue for MixSpinAmp {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for MixSpinAmp {
    const KEY_NAME: &'static str = "MIX_SPIN_AMP";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MixSpinAmp {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MIX_SPIN_AMP", CellValue::Float(self.0))
    }
}

impl ToCellValue for MixSpinAmp {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Float(self.0)
    }
}

