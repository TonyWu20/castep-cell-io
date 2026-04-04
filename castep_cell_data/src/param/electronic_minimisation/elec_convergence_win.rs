use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromKeyValue, CResult};
use castep_cell_io::query::value_as_i32;

/// Determines the size of the convergence window during a electronic minimization run.
///
/// Keyword type: Integer
///
/// Default: 3
///
/// Example:
/// ELEC_CONVERGENCE_WIN : 4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ElecConvergenceWin(pub i32);

impl Default for ElecConvergenceWin {
    fn default() -> Self {
        Self(3)
    }
}

impl FromKeyValue for ElecConvergenceWin {
    const KEY_NAME: &'static str = "ELEC_CONVERGENCE_WIN";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_i32(value)?))
    }
}

impl ToCell for ElecConvergenceWin {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("ELEC_CONVERGENCE_WIN", CellValue::Int(self.0))
    }
}

impl ToCellValue for ElecConvergenceWin {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}


