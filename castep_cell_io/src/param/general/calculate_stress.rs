use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_fmt::parse::FromKeyValue;
use castep_cell_fmt::query::value_as_bool;

/// Controls whether or not a stress calculation will be performed.
///
/// Keyword type: Logical
///
/// Default: FALSE
///
/// Example:
/// CALCULATE_STRESS : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalculateStress(pub bool);

impl FromKeyValue for CalculateStress {
    const KEY_NAME: &'static str = "CALCULATE_STRESS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl ToCell for CalculateStress {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("CALCULATE_STRESS", CellValue::Bool(self.0))
    }
}

impl ToCellValue for CalculateStress {
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
        assert_eq!(CalculateStress::from_cell_value_kv(&val).unwrap().0, true);
    }

    #[test]
    fn test_from_cell_value_false() {
        let val = CellValue::Bool(false);
        assert_eq!(CalculateStress::from_cell_value_kv(&val).unwrap().0, false);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(CalculateStress::KEY_NAME, "CALCULATE_STRESS");
    }
}

