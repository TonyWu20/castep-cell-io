use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult};
use castep_cell_fmt::query::value_as_f64;
use serde::{Deserialize, Serialize};

/// Force tolerance for convergence in the transition state search.
///
/// Keyword type: Real
///
/// Default: 0.005
///
/// Example:
/// TSSEARCH_FORCE_TOL : 0.005
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "TSSEARCH_FORCE_TOL")]
pub struct TssearchForceTol(pub f64);

impl Default for TssearchForceTol {
    fn default() -> Self {
        Self(0.005)
    }
}

impl FromCellValue for TssearchForceTol {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for TssearchForceTol {
    const KEY_NAME: &'static str = "TSSEARCH_FORCE_TOL";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for TssearchForceTol {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("TSSEARCH_FORCE_TOL", CellValue::Float(self.0))
    }
}

impl ToCellValue for TssearchForceTol {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Float(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Float(0.005);
        let result = TssearchForceTol::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 0.005);
    }

    #[test]
    fn test_default() {
        assert_eq!(TssearchForceTol::default().0, 0.005);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(TssearchForceTol::KEY_NAME, "TSSEARCH_FORCE_TOL");
    }

    #[test]
    fn test_to_cell_value() {
        let val = TssearchForceTol(0.005);
        assert_eq!(val.to_cell_value(), CellValue::Float(0.005));
    }

    #[test]
    fn test_to_cell() {
        let val = TssearchForceTol(0.005);
        match val.to_cell() {
            Cell::KeyValue(key, CellValue::Float(v)) => {
                assert_eq!(key, "TSSEARCH_FORCE_TOL");
                assert_eq!(v, 0.005);
            }
            _ => panic!("Expected KeyValue cell"),
        }
    }
}
