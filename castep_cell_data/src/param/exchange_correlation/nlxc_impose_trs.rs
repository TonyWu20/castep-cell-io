use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult};
use castep_cell_io::query::value_as_bool;

/// Controls whether time-reversal symmetry is imposed in non-local exchange-correlation.
///
/// Keyword type: Logical
///
/// Default: FALSE
///
/// Aliases: IMPOSE_TRS
///
/// Example:
/// NLXC_IMPOSE_TRS : FALSE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct NlxcImposeTrs(pub bool);

impl FromCellValue for NlxcImposeTrs {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl FromKeyValue for NlxcImposeTrs {
    const KEY_NAME: &'static str = "IMPOSE_TRS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for NlxcImposeTrs {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("NLXC_IMPOSE_TRS", CellValue::Bool(self.0))
    }
}

impl ToCellValue for NlxcImposeTrs {
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
        assert_eq!(NlxcImposeTrs::from_cell_value(&val).unwrap().0, true);
    }

    #[test]
    fn test_from_cell_value_false() {
        let val = CellValue::Bool(false);
        assert_eq!(NlxcImposeTrs::from_cell_value(&val).unwrap().0, false);
    }

    #[test]
    fn test_default() {
        assert_eq!(NlxcImposeTrs::default().0, false);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(NlxcImposeTrs::KEY_NAME, "IMPOSE_TRS");
    }

    #[test]
    fn test_to_cell() {
        let trs = NlxcImposeTrs(true);
        match trs.to_cell() {
            Cell::KeyValue(key, CellValue::Bool(val)) => {
                assert_eq!(key, "NLXC_IMPOSE_TRS");
                assert_eq!(val, true);
            }
            _ => panic!("Expected KeyValue cell"),
        }
    }
}
