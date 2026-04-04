use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromKeyValue, CResult};
use castep_cell_io::query::value_as_bool;

/// Specifies whether or not the occupancies of the bands should be fixed.
///
/// Keyword type: Logical
///
/// Default: false
///
/// Example:
/// FIX_OCCUPANCY : TRUE
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixOccupancy(pub bool);

impl FromKeyValue for FixOccupancy {
    const KEY_NAME: &'static str = "FIX_OCCUPANCY";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl ToCell for FixOccupancy {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("FIX_OCCUPANCY", CellValue::Bool(self.0))
    }
}

impl ToCellValue for FixOccupancy {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}


