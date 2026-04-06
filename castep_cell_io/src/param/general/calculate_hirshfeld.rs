use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_fmt::parse::FromKeyValue;
use castep_cell_fmt::query::value_as_bool;

/// Controls whether or not a calculation of Hirshfeld atomic charges
/// will be performed.
///
/// Keyword type: Logical
///
/// Default: FALSE
///
/// Example:
/// CALCULATE_HIRSHFELD : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalculateHirshfeld(pub bool);

impl FromKeyValue for CalculateHirshfeld {
    const KEY_NAME: &'static str = "CALCULATE_HIRSHFELD";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl ToCell for CalculateHirshfeld {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("CALCULATE_HIRSHFELD", CellValue::Bool(self.0))
    }
}

impl ToCellValue for CalculateHirshfeld {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Bool(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value_true() {
        let val = CellValue::Bool(true);
        assert_eq!(CalculateHirshfeld::from_cell_value_kv(&val).unwrap().0, true);
    }

    #[test]
    fn test_from_cell_value_false() {
        let val = CellValue::Bool(false);
        assert_eq!(CalculateHirshfeld::from_cell_value_kv(&val).unwrap().0, false);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(CalculateHirshfeld::KEY_NAME, "CALCULATE_HIRSHFELD");
    }
}

