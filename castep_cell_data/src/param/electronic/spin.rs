use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_f64;

/// Determines the initial value for the number of unpaired electrons
/// in a spin-polarized calculation.
///
/// Keyword type: Real
///
/// Default:
/// 0.0 when the total number of electrons is even.
/// 1.0 when the total number of electrons is odd.
///
/// Example:
/// SPIN : 3.0
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Spin(pub f64);

impl FromCellValue for Spin {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for Spin {
    const KEY_NAME: &'static str = "SPIN";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for Spin {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("SPIN", CellValue::Float(self.0))
    }
}

impl ToCellValue for Spin {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Float(self.0)
    }
}

