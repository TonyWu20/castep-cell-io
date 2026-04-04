use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_io::parse::FromKeyValue;
use castep_cell_io::query::value_as_bool;

/// Controls whether or not a stress calculation will be performed.
///
/// Keyword type: Logical
///
/// Default: FALSE
///
/// Example:
/// CALCULATE_STRESS : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalculateStress(pub bool);

impl FromKeyValue for CalculateStress {
    const KEY_NAME: &'static str = "CALCULATE_STRESS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl ToCell for CalculateStress {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("CALCULATE_STRESS", CellValue::Bool(self.0))
    }
}

impl ToCellValue for CalculateStress {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}


