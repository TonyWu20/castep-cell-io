use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult};
use castep_cell_io::query::value_as_f64;

/// Short-range scaling parameter for TS dispersion correction.
///
/// Keyword type: Real
///
/// Default: 1.211
///
/// Example:
/// SEDC_SR_TS : 1.211
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct SedcSrTs(pub f64);

impl FromCellValue for SedcSrTs {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for SedcSrTs {
    const KEY_NAME: &'static str = "SEDC_SR_TS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for SedcSrTs {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("SEDC_SR_TS", CellValue::Float(self.0))
    }
}

impl ToCellValue for SedcSrTs {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Float(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Float(1.211);
        let result = SedcSrTs::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 1.211);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(SedcSrTs::KEY_NAME, "SEDC_SR_TS");
    }
}
