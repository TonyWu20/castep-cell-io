use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_io::parse::FromKeyValue;
use castep_cell_io::query::value_as_bool;

/// Specifies whether the final electron density is written to an ASCII file.
///
/// Keyword type: Logical
///
/// Default: FALSE
///
/// Example:
/// WRITE_FORMATTED_DENSITY : TRUE
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct WriteFormattedDensity(pub bool);

impl FromKeyValue for WriteFormattedDensity {
    const KEY_NAME: &'static str = "WRITE_FORMATTED_DENSITY";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl ToCell for WriteFormattedDensity {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("WRITE_FORMATTED_DENSITY", CellValue::Bool(self.0))
    }
}

impl ToCellValue for WriteFormattedDensity {
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
        assert_eq!(WriteFormattedDensity::from_cell_value_kv(&val).unwrap().0, true);
    }

    #[test]
    fn test_from_cell_value_false() {
        let val = CellValue::Bool(false);
        assert_eq!(WriteFormattedDensity::from_cell_value_kv(&val).unwrap().0, false);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(WriteFormattedDensity::KEY_NAME, "WRITE_FORMATTED_DENSITY");
    }
}

