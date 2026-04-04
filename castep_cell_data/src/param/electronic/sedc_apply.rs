use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult};
use castep_cell_io::query::value_as_bool;

/// Controls whether to apply semi-empirical dispersion correction.
///
/// Keyword type: Logical
///
/// Default: FALSE
///
/// Example:
/// SEDC_APPLY : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SedcApply(pub bool);

impl FromCellValue for SedcApply {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl FromKeyValue for SedcApply {
    const KEY_NAME: &'static str = "SEDC_APPLY";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for SedcApply {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("SEDC_APPLY", CellValue::Bool(self.0))
    }
}

impl ToCellValue for SedcApply {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value_true() {
        let val = CellValue::Bool(true);
        assert_eq!(SedcApply::from_cell_value(&val).unwrap().0, true);
    }

    #[test]
    fn test_from_cell_value_false() {
        let val = CellValue::Bool(false);
        assert_eq!(SedcApply::from_cell_value(&val).unwrap().0, false);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(SedcApply::KEY_NAME, "SEDC_APPLY");
    }

    #[test]
    fn test_default() {
        assert_eq!(SedcApply::default().0, false);
    }
}
