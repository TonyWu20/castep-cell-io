use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult};
use castep_cell_io::query::value_as_f64;
use serde::{Deserialize, Serialize};

/// Displacement tolerance for convergence in the transition state search.
///
/// Keyword type: Real
///
/// Default: 0.01
///
/// Example:
/// TSSEARCH_DISP_TOL : 0.01
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "TSSEARCH_DISP_TOL")]
pub struct TssearchDispTol(pub f64);

impl Default for TssearchDispTol {
    fn default() -> Self {
        Self(0.01)
    }
}

impl FromCellValue for TssearchDispTol {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for TssearchDispTol {
    const KEY_NAME: &'static str = "TSSEARCH_DISP_TOL";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for TssearchDispTol {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("TSSEARCH_DISP_TOL", CellValue::Float(self.0))
    }
}

impl ToCellValue for TssearchDispTol {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Float(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Float(0.01);
        let result = TssearchDispTol::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 0.01);
    }

    #[test]
    fn test_default() {
        assert_eq!(TssearchDispTol::default().0, 0.01);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(TssearchDispTol::KEY_NAME, "TSSEARCH_DISP_TOL");
    }

    #[test]
    fn test_to_cell_value() {
        let val = TssearchDispTol(0.01);
        assert_eq!(val.to_cell_value(), CellValue::Float(0.01));
    }

    #[test]
    fn test_to_cell() {
        let val = TssearchDispTol(0.01);
        match val.to_cell() {
            Cell::KeyValue(key, CellValue::Float(v)) => {
                assert_eq!(key, "TSSEARCH_DISP_TOL");
                assert_eq!(v, 0.01);
            }
            _ => panic!("Expected KeyValue cell"),
        }
    }
}
