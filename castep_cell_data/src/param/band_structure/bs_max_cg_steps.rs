use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult};
use castep_cell_io::query::value_as_u32;
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
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("BS_MAX_CG_STEPS", CellValue::UInt(self.0))
    }
}

impl ToCellValue for BsMaxCgSteps {
    fn to_cell_value(&self) -> CellValue {
        CellValue::UInt(self.0)
    }
}

