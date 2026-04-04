use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_i32;
use serde::{Deserialize, Serialize};

/// Determines the number of geometry optimization steps for which the total spin is fixed.
///
/// Keyword type: Integer
///
/// Default: 0 (spin is allowed to vary)
///
/// Example:
/// GEOM_SPIN_FIX : 5
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "GEOM_SPIN_FIX")]
pub struct GeomSpinFix(pub i32); // Using i32 to allow negative values

impl FromCellValue for GeomSpinFix {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_i32(value)?))
    }
}

impl FromKeyValue for GeomSpinFix {
    const KEY_NAME: &'static str = "GEOM_SPIN_FIX";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for GeomSpinFix {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("GEOM_SPIN_FIX", CellValue::Int(self.0))
    }
}

impl ToCellValue for GeomSpinFix {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}


