use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult};
use castep_cell_fmt::query::value_as_f64;

/// Damping parameter for JCHS dispersion correction.
///
/// Keyword type: Real
///
/// Default: 0.44
///
/// Example:
/// SEDC_D_JCHS : 0.44
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct SedcDJchs(pub f64);

impl FromCellValue for SedcDJchs {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for SedcDJchs {
    const KEY_NAME: &'static str = "SEDC_D_JCHS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for SedcDJchs {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("SEDC_D_JCHS", CellValue::Float(self.0))
    }
}

impl ToCellValue for SedcDJchs {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Float(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Float(0.44);
        let result = SedcDJchs::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 0.44);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(SedcDJchs::KEY_NAME, "SEDC_D_JCHS");
    }
}
