use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult};
use castep_cell_io::query::value_as_u32;
use serde::{Deserialize, Serialize};

/// Controls the maximum number of iterations to perform when calculating band structure.
///
/// Keyword type: Integer
///
/// Default: 60
///
/// Example:
/// BS_MAX_ITER : 50
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "BS_MAX_ITER")]
pub struct BsMaxIter(pub u32);

impl Default for BsMaxIter {
    fn default() -> Self {
        Self(60)
    }
}

impl FromCellValue for BsMaxIter {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for BsMaxIter {
    const KEY_NAME: &'static str = "BS_MAX_ITER";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for BsMaxIter {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("BS_MAX_ITER", CellValue::UInt(self.0))
    }
}

impl ToCellValue for BsMaxIter {
    fn to_cell_value(&self) -> CellValue {
        CellValue::UInt(self.0)
    }
}

