use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_bool;

/// Specifies whether or not to perform a population analysis on the final ground state.
///
/// Keyword type: Logical
///
/// Default: true
///
/// Example:
/// POPN_CALCULATE : FALSE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PopnCalculate(pub bool);

impl Default for PopnCalculate {
    fn default() -> Self {
        Self(true)
    }
}

impl FromCellValue for PopnCalculate {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl FromKeyValue for PopnCalculate {
    const KEY_NAME: &'static str = "POPN_CALCULATE";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for PopnCalculate {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("POPN_CALCULATE", CellValue::Bool(self.0))
    }
}

impl ToCellValue for PopnCalculate {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}

