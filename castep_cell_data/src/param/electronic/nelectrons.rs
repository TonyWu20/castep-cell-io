use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_f64;

/// Specifies the total number of electrons in the system.
///
/// Keyword type: Real
///
/// Default:
/// If CHARGE is specified, NELECTRONS is chosen to match the charge.
/// If NUP and NDOWN are specified, NELECTRONS = NUP + NDOWN.
/// Otherwise, a default value for a neutral system is chosen.
///
/// Example:
/// NELECTRONS : 3.0
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Nelectrons(pub f64);

impl FromCellValue for Nelectrons {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for Nelectrons {
    const KEY_NAME: &'static str = "NELECTRONS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for Nelectrons {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("NELECTRONS", CellValue::Float(self.0))
    }
}

impl ToCellValue for Nelectrons {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Float(self.0)
    }
}

