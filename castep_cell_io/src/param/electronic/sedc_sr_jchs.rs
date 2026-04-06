use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult};
use castep_cell_fmt::query::value_as_f64;

/// Short-range scaling parameter for JCHS dispersion correction.
///
/// Keyword type: Real
///
/// Default: 0.722
///
/// Example:
/// SEDC_SR_JCHS : 0.722
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct SedcSrJchs(pub f64);

impl FromCellValue for SedcSrJchs {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for SedcSrJchs {
    const KEY_NAME: &'static str = "SEDC_SR_JCHS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for SedcSrJchs {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("SEDC_SR_JCHS", CellValue::Float(self.0))
    }
}

impl ToCellValue for SedcSrJchs {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Float(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Float(0.722);
        let result = SedcSrJchs::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 0.722);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(SedcSrJchs::KEY_NAME, "SEDC_SR_JCHS");
    }
}
