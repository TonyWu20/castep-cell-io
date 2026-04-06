use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult};
use castep_cell_fmt::query::value_as_f64;
use serde::{Deserialize, Serialize};

/// Determines the size of the standard grid, relative to the diameter
/// of the cutoff sphere.
///
/// Keyword type: Real
///
/// Default: 1.75
///
/// Example:
/// GRID_SCALE : 2.0
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "GRID_SCALE")]
pub struct GridScale(pub f64);

impl Default for GridScale {
    fn default() -> Self {
        Self(1.75)
    }
}

impl FromCellValue for GridScale {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for GridScale {
    const KEY_NAME: &'static str = "GRID_SCALE";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for GridScale {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("GRID_SCALE", CellValue::Float(self.0))
    }
}

impl ToCellValue for GridScale {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Float(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Float(2.0);
        let result = GridScale::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 2.0);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(GridScale::KEY_NAME, "GRID_SCALE");
    }
}

