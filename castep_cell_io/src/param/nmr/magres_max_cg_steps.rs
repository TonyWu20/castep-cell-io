use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_u32;

/// Controls the maximum number of conjugate gradient steps during an NMR calculation.
///
/// Keyword type: Integer
///
/// Default: 250
///
/// Example:
/// MAGRES_MAX_CG_STEPS : 300
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MagresMaxCgSteps(pub u32);

impl Default for MagresMaxCgSteps {
    fn default() -> Self {
        Self(250)
    }
}

impl FromCellValue for MagresMaxCgSteps {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for MagresMaxCgSteps {
    const KEY_NAME: &'static str = "MAGRES_MAX_CG_STEPS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MagresMaxCgSteps {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MAGRES_MAX_CG_STEPS", CellValue::UInt(self.0))
    }
}

impl ToCellValue for MagresMaxCgSteps {
    fn to_cell_value(&self) -> CellValue {
        CellValue::UInt(self.0)
    }
}

