use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_f64;

/// Specifies the total charge of the system.
///
/// Keyword type: Real
///
/// Default: 0.0
///
/// Example:
/// CHARGE : 3.0
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Charge(pub f64);

impl Default for Charge {
    fn default() -> Self {
        Self(0.0)
    }
}

impl FromCellValue for Charge {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for Charge {
    const KEY_NAME: &'static str = "CHARGE";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for Charge {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("CHARGE", CellValue::Float(self.0))
    }
}

impl ToCellValue for Charge {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Float(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Float(2.5);
        let result = Charge::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 2.5);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(Charge::KEY_NAME, "CHARGE");
    }
}
