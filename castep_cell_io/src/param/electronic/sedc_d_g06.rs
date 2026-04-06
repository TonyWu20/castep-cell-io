use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult};
use castep_cell_fmt::query::value_as_f64;

/// Damping parameter for G06 dispersion correction.
///
/// Keyword type: Real
///
/// Default: 0.75
///
/// Example:
/// SEDC_D_G06 : 0.75
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct SedcDG06(pub f64);

impl FromCellValue for SedcDG06 {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for SedcDG06 {
    const KEY_NAME: &'static str = "SEDC_D_G06";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for SedcDG06 {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("SEDC_D_G06", CellValue::Float(self.0))
    }
}

impl ToCellValue for SedcDG06 {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Float(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Float(0.75);
        let result = SedcDG06::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 0.75);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(SedcDG06::KEY_NAME, "SEDC_D_G06");
    }
}
