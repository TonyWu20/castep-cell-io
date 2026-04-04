use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromKeyValue, CResult};
use castep_cell_fmt::query::value_as_bool;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value_true() {
        let val = CellValue::Bool(true);
        assert_eq!(FixOccupancy::from_cell_value_kv(&val).unwrap().0, true);
    }

    #[test]
    fn test_from_cell_value_false() {
        let val = CellValue::Bool(false);
        assert_eq!(FixOccupancy::from_cell_value_kv(&val).unwrap().0, false);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(FixOccupancy::KEY_NAME, "FIX_OCCUPANCY");
    }
}

