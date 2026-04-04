use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_fmt::parse::FromKeyValue;
use castep_cell_fmt::query::value_as_i32;

/// Specifies the maximum run time for the job, in seconds.
///
/// Keyword type: Integer
///
/// Default: 0 (no time limit)
///
/// Example:
/// RUN_TIME : 360
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RunTime(pub i32); // i32 to allow <= 0 values

impl FromKeyValue for RunTime {
    const KEY_NAME: &'static str = "RUN_TIME";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_i32(value)?))
    }
}

impl ToCell for RunTime {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("RUN_TIME", CellValue::Int(self.0))
    }
}

impl ToCellValue for RunTime {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}


