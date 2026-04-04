use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult};
use castep_cell_fmt::query::value_as_f64;

/// Scaling parameter for JCHS dispersion correction.
///
/// Keyword type: Real
///
/// Default: 1.0
///
/// Example:
/// SEDC_S6_JCHS : 1.0
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct SedcS6Jchs(pub f64);

impl FromCellValue for SedcS6Jchs {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for SedcS6Jchs {
    const KEY_NAME: &'static str = "SEDC_S6_JCHS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for SedcS6Jchs {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("SEDC_S6_JCHS", CellValue::Float(self.0))
    }
}

impl ToCellValue for SedcS6Jchs {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Float(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Float(1.0);
        let result = SedcS6Jchs::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 1.0);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(SedcS6Jchs::KEY_NAME, "SEDC_S6_JCHS");
    }
}
