use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_f64;

/// Determines the total number of down-spin electrons.
/// Used only if SPIN_POLARIZED = TRUE.
///
/// Keyword type: Real
///
/// Default:
/// If SPIN is specified: NDOWN = (NELECTRONS - SPIN) / 2
/// Else: NDOWN = NELECTRONS / 2
///
/// Example:
/// NDOWN : 12.0
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Ndown(pub f64);

impl FromCellValue for Ndown {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for Ndown {
    const KEY_NAME: &'static str = "NDOWN";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for Ndown {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("NDOWN", CellValue::Float(self.0))
    }
}

impl ToCellValue for Ndown {
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
        let result = Ndown::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 12.0);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(Ndown::KEY_NAME, "NDOWN");
    }
}
