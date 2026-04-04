use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult};
use castep_cell_io::query::value_as_u32;
use serde::{Deserialize, Serialize};

/// Maximum number of points along the reaction path in the transition state search.
///
/// Keyword type: Integer
///
/// Default: 10
///
/// Example:
/// TSSEARCH_MAX_PATH_POINTS : 10
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "TSSEARCH_MAX_PATH_POINTS")]
pub struct TssearchMaxPathPoints(pub u32);

impl Default for TssearchMaxPathPoints {
    fn default() -> Self {
        Self(10)
    }
}

impl FromCellValue for TssearchMaxPathPoints {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for TssearchMaxPathPoints {
    const KEY_NAME: &'static str = "TSSEARCH_MAX_PATH_POINTS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for TssearchMaxPathPoints {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("TSSEARCH_MAX_PATH_POINTS", CellValue::UInt(self.0))
    }
}

impl ToCellValue for TssearchMaxPathPoints {
    fn to_cell_value(&self) -> CellValue {
        CellValue::UInt(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::UInt(10);
        let result = TssearchMaxPathPoints::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 10);
    }

    #[test]
    fn test_default() {
        assert_eq!(TssearchMaxPathPoints::default().0, 10);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(TssearchMaxPathPoints::KEY_NAME, "TSSEARCH_MAX_PATH_POINTS");
    }

    #[test]
    fn test_to_cell_value() {
        let val = TssearchMaxPathPoints(10);
        assert_eq!(val.to_cell_value(), CellValue::UInt(10));
    }

    #[test]
    fn test_to_cell() {
        let val = TssearchMaxPathPoints(10);
        match val.to_cell() {
            Cell::KeyValue(key, CellValue::UInt(v)) => {
                assert_eq!(key, "TSSEARCH_MAX_PATH_POINTS");
                assert_eq!(v, 10);
            }
            _ => panic!("Expected KeyValue cell"),
        }
    }
}
