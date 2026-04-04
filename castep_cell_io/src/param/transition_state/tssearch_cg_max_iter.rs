use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult};
use castep_cell_fmt::query::value_as_u32;
use serde::{Deserialize, Serialize};

/// Maximum number of conjugate gradient iterations in the transition state search.
///
/// Keyword type: Integer
///
/// Default: 20
///
/// Example:
/// TSSEARCH_CG_MAX_ITER : 20
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "TSSEARCH_CG_MAX_ITER")]
pub struct TssearchCgMaxIter(pub u32);

impl Default for TssearchCgMaxIter {
    fn default() -> Self {
        Self(20)
    }
}

impl FromCellValue for TssearchCgMaxIter {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for TssearchCgMaxIter {
    const KEY_NAME: &'static str = "TSSEARCH_CG_MAX_ITER";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for TssearchCgMaxIter {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("TSSEARCH_CG_MAX_ITER", CellValue::UInt(self.0))
    }
}

impl ToCellValue for TssearchCgMaxIter {
    fn to_cell_value(&self) -> CellValue {
        CellValue::UInt(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::UInt(20);
        let result = TssearchCgMaxIter::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 20);
    }

    #[test]
    fn test_default() {
        assert_eq!(TssearchCgMaxIter::default().0, 20);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(TssearchCgMaxIter::KEY_NAME, "TSSEARCH_CG_MAX_ITER");
    }

    #[test]
    fn test_to_cell_value() {
        let val = TssearchCgMaxIter(20);
        assert_eq!(val.to_cell_value(), CellValue::UInt(20));
    }

    #[test]
    fn test_to_cell() {
        let val = TssearchCgMaxIter(20);
        match val.to_cell() {
            Cell::KeyValue(key, CellValue::UInt(v)) => {
                assert_eq!(key, "TSSEARCH_CG_MAX_ITER");
                assert_eq!(v, 20);
            }
            _ => panic!("Expected KeyValue cell"),
        }
    }
}
