use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult};
use castep_cell_io::query::value_as_f64;

/// Scaling parameter for G06 dispersion correction.
///
/// Keyword type: Real
///
/// Default: 1.05
///
/// Example:
/// SEDC_S6_G06 : 1.05
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct SedcS6G06(pub f64);

impl FromCellValue for SedcS6G06 {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for SedcS6G06 {
    const KEY_NAME: &'static str = "SEDC_S6_G06";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for SedcS6G06 {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("SEDC_S6_G06", CellValue::Float(self.0))
    }
}

impl ToCellValue for SedcS6G06 {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Float(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Float(1.05);
        let result = SedcS6G06::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 1.05);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(SedcS6G06::KEY_NAME, "SEDC_S6_G06");
    }
}
