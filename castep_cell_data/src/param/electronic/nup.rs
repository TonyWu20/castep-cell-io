use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_f64;

/// Determines the total number of up-spin electrons.
/// Used only if SPIN_POLARIZED = TRUE.
///
/// Keyword type: Real
///
/// Default:
/// If SPIN is specified: NUP = (NELECTRONS + SPIN) / 2
/// Else: NUP = NELECTRONS / 2
///
/// Example:
/// NUP : 12.0
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Nup(pub f64);

impl FromCellValue for Nup {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for Nup {
    const KEY_NAME: &'static str = "NUP";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for Nup {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("NUP", CellValue::Float(self.0))
    }
}

impl ToCellValue for Nup {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Float(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Float(12.0);
        let result = Nup::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 12.0);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(Nup::KEY_NAME, "NUP");
    }
}
