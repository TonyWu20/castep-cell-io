use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult};
use castep_cell_io::query::value_as_u32;
use serde::{Deserialize, Serialize};

/// Maximum number of quasi-static transitions iterations in the transition state search.
///
/// Keyword type: Integer
///
/// Default: 5
///
/// Example:
/// TSSEARCH_QST_MAX_ITER : 5
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "TSSEARCH_QST_MAX_ITER")]
pub struct TssearchQstMaxIter(pub u32);

impl Default for TssearchQstMaxIter {
    fn default() -> Self {
        Self(5)
    }
}

impl FromCellValue for TssearchQstMaxIter {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for TssearchQstMaxIter {
    const KEY_NAME: &'static str = "TSSEARCH_QST_MAX_ITER";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for TssearchQstMaxIter {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("TSSEARCH_QST_MAX_ITER", CellValue::UInt(self.0))
    }
}

impl ToCellValue for TssearchQstMaxIter {
    fn to_cell_value(&self) -> CellValue {
        CellValue::UInt(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::UInt(5);
        let result = TssearchQstMaxIter::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 5);
    }

    #[test]
    fn test_default() {
        assert_eq!(TssearchQstMaxIter::default().0, 5);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(TssearchQstMaxIter::KEY_NAME, "TSSEARCH_QST_MAX_ITER");
    }

    #[test]
    fn test_to_cell_value() {
        let val = TssearchQstMaxIter(5);
        assert_eq!(val.to_cell_value(), CellValue::UInt(5));
    }

    #[test]
    fn test_to_cell() {
        let val = TssearchQstMaxIter(5);
        match val.to_cell() {
            Cell::KeyValue(key, CellValue::UInt(v)) => {
                assert_eq!(key, "TSSEARCH_QST_MAX_ITER");
                assert_eq!(v, 5);
            }
            _ => panic!("Expected KeyValue cell"),
        }
    }
}
