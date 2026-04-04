use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_u32;
use serde::{Deserialize, Serialize};

/// Determines the maximum number of steps in a geometry optimization.
///
/// Keyword type: Integer
///
/// Default: 30
///
/// Example:
/// GEOM_MAX_ITER : 25
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "GEOM_MAX_ITER")]
pub struct GeomMaxIter(pub u32);

impl FromCellValue for GeomMaxIter {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for GeomMaxIter {
    const KEY_NAME: &'static str = "GEOM_MAX_ITER";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl Default for GeomMaxIter {
    fn default() -> Self {
        Self(30) // Default is 30
    }
}

impl ToCell for GeomMaxIter {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("GEOM_MAX_ITER", CellValue::UInt(self.0))
    }
}

impl ToCellValue for GeomMaxIter {
    fn to_cell_value(&self) -> CellValue {
        CellValue::UInt(self.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::UInt(25);
        let result = GeomMaxIter::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 25);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(GeomMaxIter::KEY_NAME, "GEOM_MAX_ITER");
    }
}

