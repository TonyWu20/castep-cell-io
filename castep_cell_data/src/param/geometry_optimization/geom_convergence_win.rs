// File: geometry_optimization/geom_convergence_win.rs
use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_i32;
use serde::{Deserialize, Serialize};

/// Determines the size of the convergence window for a geometry optimization.
///
/// Keyword type: Integer
///
/// Default: 2
///
/// Example:
/// GEOM_CONVERGENCE_WIN : 4
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "GEOM_CONVERGENCE_WIN")]
pub struct GeomConvergenceWin(pub i32); // Using i32

impl FromCellValue for GeomConvergenceWin {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_i32(value)?))
    }
}

impl FromKeyValue for GeomConvergenceWin {
    const KEY_NAME: &'static str = "GEOM_CONVERGENCE_WIN";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl Default for GeomConvergenceWin {
    fn default() -> Self {
        Self(2) // Default is 2
    }
}

impl ToCell for GeomConvergenceWin {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("GEOM_CONVERGENCE_WIN", CellValue::Int(self.0))
    }
}

impl ToCellValue for GeomConvergenceWin {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}


