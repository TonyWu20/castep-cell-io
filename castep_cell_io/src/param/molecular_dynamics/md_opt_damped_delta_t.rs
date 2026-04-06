use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_bool;

/// Controls whether or not the optimal time step will be calculated after each damped MD step.
///
/// Keyword type: Logical
///
/// Default: false
///
/// Example:
/// MD_OPT_DAMPED_DELTA_T : TRUE
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MdOptDampedDeltaT(pub bool);

impl FromCellValue for MdOptDampedDeltaT {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl FromKeyValue for MdOptDampedDeltaT {
    const KEY_NAME: &'static str = "MD_OPT_DAMPED_DELTA_T";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MdOptDampedDeltaT {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("MD_OPT_DAMPED_DELTA_T", self.to_cell_value())
    }
}

impl ToCellValue for MdOptDampedDeltaT {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Bool(self.0)
    }
}


