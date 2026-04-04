use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromKeyValue, CResult};
use castep_cell_fmt::query::value_as_i32;

/// Determines the number of electronic iterations for which the total spin is fixed.
///
/// Keyword type: Integer
///
/// Default: 10
///
/// Example:
/// SPIN_FIX : 5
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpinFix(pub i32);

impl Default for SpinFix {
    fn default() -> Self {
        Self(10)
    }
}

impl FromKeyValue for SpinFix {
    const KEY_NAME: &'static str = "SPIN_FIX";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_i32(value)?))
    }
}

impl ToCell for SpinFix {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("SPIN_FIX", CellValue::Int(self.0))
    }
}

impl ToCellValue for SpinFix {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}


