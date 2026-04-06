use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_bool;
use serde::{Deserialize, Serialize};

/// Specifies whether to compute Born effective charge tensors.
///
/// Keyword type: Logical
///
/// Default: true
///
/// Example:
/// CALCULATE_BORN_CHARGES : FALSE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "CALCULATE_BORN_CHARGES")]
pub struct CalculateBornCharges(pub bool);

impl FromCellValue for CalculateBornCharges {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl FromKeyValue for CalculateBornCharges {
    const KEY_NAME: &'static str = "CALCULATE_BORN_CHARGES";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl Default for CalculateBornCharges {
    fn default() -> Self {
        Self(true) // Default is TRUE
    }
}

impl ToCell for CalculateBornCharges {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("CALCULATE_BORN_CHARGES", CellValue::Bool(self.0))
    }
}

impl ToCellValue for CalculateBornCharges {
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
        assert_eq!(CalculateBornCharges::from_cell_value(&val).unwrap().0, true);
    }

    #[test]
    fn test_from_cell_value_false() {
        let val = CellValue::Bool(false);
        assert_eq!(CalculateBornCharges::from_cell_value(&val).unwrap().0, false);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(CalculateBornCharges::KEY_NAME, "CALCULATE_BORN_CHARGES");
    }
}

