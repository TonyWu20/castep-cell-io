use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_u32;

/// Specifies the number of MD steps between recalculations of damping parameters.
///
/// Keyword type: Integer
///
/// Default: 30
///
/// Example:
/// MD_DAMPING_RESET : 20
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MdDampingReset(pub u32); // Using u32 as it's a count of steps

impl Default for MdDampingReset {
    fn default() -> Self {
        Self(30) // Default is 30
    }
}

impl FromCellValue for MdDampingReset {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for MdDampingReset {
    const KEY_NAME: &'static str = "MD_DAMPING_RESET";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MdDampingReset {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_DAMPING_RESET", CellValue::UInt(self.0))
    }
}

impl ToCellValue for MdDampingReset {
    fn to_cell_value(&self) -> CellValue {
        CellValue::UInt(self.0)
    }
}


