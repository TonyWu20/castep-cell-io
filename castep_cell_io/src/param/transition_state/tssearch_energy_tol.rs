use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult};
use castep_cell_fmt::query::value_as_f64;
use serde::{Deserialize, Serialize};

/// Energy tolerance for convergence in the transition state search.
///
/// Keyword type: Real
///
/// Default: 0.00002
///
/// Example:
/// TSSEARCH_ENERGY_TOL : 0.00002
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "TSSEARCH_ENERGY_TOL")]
pub struct TssearchEnergyTol(pub f64);

impl Default for TssearchEnergyTol {
    fn default() -> Self {
        Self(0.00002)
    }
}

impl FromCellValue for TssearchEnergyTol {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for TssearchEnergyTol {
    const KEY_NAME: &'static str = "TSSEARCH_ENERGY_TOL";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for TssearchEnergyTol {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("TSSEARCH_ENERGY_TOL", CellValue::Float(self.0))
    }
}

impl ToCellValue for TssearchEnergyTol {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Float(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Float(0.00002);
        let result = TssearchEnergyTol::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 0.00002);
    }

    #[test]
    fn test_default() {
        assert_eq!(TssearchEnergyTol::default().0, 0.00002);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(TssearchEnergyTol::KEY_NAME, "TSSEARCH_ENERGY_TOL");
    }

    #[test]
    fn test_to_cell_value() {
        let val = TssearchEnergyTol(0.00002);
        assert_eq!(val.to_cell_value(), CellValue::Float(0.00002));
    }

    #[test]
    fn test_to_cell() {
        let val = TssearchEnergyTol(0.00002);
        match val.to_cell() {
            Cell::KeyValue(key, CellValue::Float(v)) => {
                assert_eq!(key, "TSSEARCH_ENERGY_TOL");
                assert_eq!(v, 0.00002);
            }
            _ => panic!("Expected KeyValue cell"),
        }
    }
}
