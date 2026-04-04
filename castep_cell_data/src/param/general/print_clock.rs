use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_io::parse::FromKeyValue;
use castep_cell_io::query::value_as_bool;

/// Specifies whether or not timing information will be printed.
///
/// Keyword type: Logical
///
/// Default: TRUE
///
/// Example:
/// PRINT_CLOCK : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintClock(pub bool);

impl Default for PrintClock {
    fn default() -> Self {
        Self(true) // Default is TRUE
    }
}

impl FromKeyValue for PrintClock {
    const KEY_NAME: &'static str = "PRINT_CLOCK";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl ToCell for PrintClock {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PRINT_CLOCK", CellValue::Bool(self.0))
    }
}

impl ToCellValue for PrintClock {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}


