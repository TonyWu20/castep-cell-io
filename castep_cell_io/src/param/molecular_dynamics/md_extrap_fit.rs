use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_bool;

/// Controls whether or not best fit extrapolation parameters will be calculated.
///
/// Keyword type: Logical
///
/// Default: true
///
/// Example:
/// MD_EXTRAP_FIT = FALSE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MdExtrapFit(pub bool);

impl Default for MdExtrapFit {
    fn default() -> Self {
        Self(true) // Default is TRUE
    }
}

impl FromCellValue for MdExtrapFit {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl FromKeyValue for MdExtrapFit {
    const KEY_NAME: &'static str = "MD_EXTRAP_FIT";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MdExtrapFit {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_EXTRAP_FIT", CellValue::Bool(self.0))
    }
}

impl ToCellValue for MdExtrapFit {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}


