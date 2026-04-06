use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult};
use castep_cell_fmt::query::value_as_u32;
use serde::{Deserialize, Serialize};

/// Controls the maximum number of conjugate gradient steps taken for each electronic band.
///
/// Keyword type: Integer
///
/// Default: 4
///
/// Example:
/// BS_MAX_CG_STEPS : 10
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "BS_MAX_CG_STEPS")]
pub struct BsMaxCgSteps(pub u32);

impl Default for BsMaxCgSteps {
    fn default() -> Self {
        Self(4)
    }
}

impl FromCellValue for BsMaxCgSteps {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for BsMaxCgSteps {
    const KEY_NAME: &'static str = "BS_MAX_CG_STEPS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for BsMaxCgSteps {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("BS_MAX_CG_STEPS", CellValue::UInt(self.0))
    }
}

impl ToCellValue for BsMaxCgSteps {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::UInt(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::UInt(10);
        let result = BsMaxCgSteps::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 10);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(BsMaxCgSteps::KEY_NAME, "BS_MAX_CG_STEPS");
    }
}
