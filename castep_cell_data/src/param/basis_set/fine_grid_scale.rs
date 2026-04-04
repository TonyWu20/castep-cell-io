use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult};
use castep_cell_io::query::value_as_f64;
use serde::{Deserialize, Serialize};

/// Determines the maximum size of the g-vectors included in the fine grid
/// relative to the standard grid.
///
/// Keyword type: Real
///
/// Default: 1.0 (results in fine and standard grids being identical)
///
/// Example:
/// FINE_GRID_SCALE : 2.0
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "FINE_GRID_SCALE")]
pub struct FineGridScale(pub f64);

impl Default for FineGridScale {
    fn default() -> Self {
        Self(1.0)
    }
}

impl FromCellValue for FineGridScale {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for FineGridScale {
    const KEY_NAME: &'static str = "FINE_GRID_SCALE";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for FineGridScale {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("FINE_GRID_SCALE", CellValue::Float(self.0))
    }
}

impl ToCellValue for FineGridScale {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Float(self.0)
    }
}


