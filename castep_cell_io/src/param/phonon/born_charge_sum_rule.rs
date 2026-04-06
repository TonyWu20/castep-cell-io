use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_bool;
use serde::{Deserialize, Serialize};

/// Specifies whether to correct the Born effective charge tensor to enforce the sum rule.
///
/// Keyword type: Logical
///
/// Default: false
///
/// Example:
/// BORN_CHARGE_SUM_RULE : TRUE
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "BORN_CHARGE_SUM_RULE")]
pub struct BornChargeSumRule(pub bool);

impl FromCellValue for BornChargeSumRule {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl FromKeyValue for BornChargeSumRule {
    const KEY_NAME: &'static str = "BORN_CHARGE_SUM_RULE";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for BornChargeSumRule {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("BORN_CHARGE_SUM_RULE", CellValue::Bool(self.0))
    }
}

impl ToCellValue for BornChargeSumRule {
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
        assert_eq!(BornChargeSumRule::from_cell_value(&val).unwrap().0, true);
    }

    #[test]
    fn test_from_cell_value_false() {
        let val = CellValue::Bool(false);
        assert_eq!(BornChargeSumRule::from_cell_value(&val).unwrap().0, false);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(BornChargeSumRule::KEY_NAME, "BORN_CHARGE_SUM_RULE");
    }
}

