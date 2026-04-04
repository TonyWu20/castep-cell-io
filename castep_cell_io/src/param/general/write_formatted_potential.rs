use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_fmt::parse::FromKeyValue;
use castep_cell_fmt::query::value_as_bool;

/// Specifies whether the final local potential is written to an ASCII file.
///
/// Keyword type: Logical
///
/// Default: FALSE
///
/// Example:
/// WRITE_FORMATTED_POTENTIAL : TRUE
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct WriteFormattedPotential(pub bool);

impl FromKeyValue for WriteFormattedPotential {
    const KEY_NAME: &'static str = "WRITE_FORMATTED_POTENTIAL";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl ToCell for WriteFormattedPotential {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("WRITE_FORMATTED_POTENTIAL", CellValue::Bool(self.0))
    }
}

impl ToCellValue for WriteFormattedPotential {
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
        assert_eq!(WriteFormattedPotential::from_cell_value_kv(&val).unwrap().0, true);
    }

    #[test]
    fn test_from_cell_value_false() {
        let val = CellValue::Bool(false);
        assert_eq!(WriteFormattedPotential::from_cell_value_kv(&val).unwrap().0, false);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(WriteFormattedPotential::KEY_NAME, "WRITE_FORMATTED_POTENTIAL");
    }
}

