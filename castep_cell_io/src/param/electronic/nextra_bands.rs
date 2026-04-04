use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_u32;

/// Controls the number of extra bands in addition to the number of occupied bands.
///
/// Keyword type: Integer
///
/// Default: 0
///
/// Example:
/// NEXTRA_BANDS : 12
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct NextraBands(pub u32);

impl FromCellValue for NextraBands {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for NextraBands {
    const KEY_NAME: &'static str = "NEXTRA_BANDS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for NextraBands {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("NEXTRA_BANDS", CellValue::UInt(self.0))
    }
}

impl ToCellValue for NextraBands {
    fn to_cell_value(&self) -> CellValue {
        CellValue::UInt(self.0)
    }
}

