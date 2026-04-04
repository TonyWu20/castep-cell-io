use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_io::parse::FromKeyValue;
use castep_cell_io::query::value_as_bool;

/// Controls whether or not a calculation of the electron localization function
/// will be performed.
///
/// Keyword type: Logical
///
/// Default: FALSE
///
/// Example:
/// CALCULATE_ELF : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalculateElf(pub bool);

impl FromKeyValue for CalculateElf {
    const KEY_NAME: &'static str = "CALCULATE_ELF";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl ToCell for CalculateElf {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("CALCULATE_ELF", CellValue::Bool(self.0))
    }
}

impl ToCellValue for CalculateElf {
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
        assert_eq!(CalculateElf::from_cell_value_kv(&val).unwrap().0, true);
    }

    #[test]
    fn test_from_cell_value_false() {
        let val = CellValue::Bool(false);
        assert_eq!(CalculateElf::from_cell_value_kv(&val).unwrap().0, false);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(CalculateElf::KEY_NAME, "CALCULATE_ELF");
    }
}

