use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_bool;

/// Controls whether or not different wavefunctions are used for different spins.
///
/// Keyword type: Logical
///
/// Default: false
///
/// Example:
/// SPIN_POLARIZED : TRUE
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SpinPolarized(pub bool);

impl FromCellValue for SpinPolarized {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl FromKeyValue for SpinPolarized {
    const KEY_NAME: &'static str = "SPIN_POLARIZED";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for SpinPolarized {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("SPIN_POLARIZED", CellValue::Bool(self.0))
    }
}

impl ToCellValue for SpinPolarized {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}

