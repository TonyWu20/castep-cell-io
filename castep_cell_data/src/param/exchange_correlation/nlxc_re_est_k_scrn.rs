use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult};
use castep_cell_io::query::value_as_bool;

/// Controls whether k-space screening is re-estimated in non-local exchange-correlation.
///
/// Keyword type: Logical
///
/// Default: FALSE
///
/// Aliases: RE_EST_K_SCRN
///
/// Example:
/// NLXC_RE_EST_K_SCRN : FALSE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct NlxcReEstKScrn(pub bool);

impl FromCellValue for NlxcReEstKScrn {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl FromKeyValue for NlxcReEstKScrn {
    const KEY_NAME: &'static str = "RE_EST_K_SCRN";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for NlxcReEstKScrn {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("NLXC_RE_EST_K_SCRN", CellValue::Bool(self.0))
    }
}

impl ToCellValue for NlxcReEstKScrn {
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
        assert_eq!(NlxcReEstKScrn::from_cell_value(&val).unwrap().0, true);
    }

    #[test]
    fn test_from_cell_value_false() {
        let val = CellValue::Bool(false);
        assert_eq!(NlxcReEstKScrn::from_cell_value(&val).unwrap().0, false);
    }

    #[test]
    fn test_default() {
        assert_eq!(NlxcReEstKScrn::default().0, false);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(NlxcReEstKScrn::KEY_NAME, "RE_EST_K_SCRN");
    }

    #[test]
    fn test_to_cell() {
        let scrn = NlxcReEstKScrn(true);
        match scrn.to_cell() {
            Cell::KeyValue(key, CellValue::Bool(val)) => {
                assert_eq!(key, "NLXC_RE_EST_K_SCRN");
                assert_eq!(val, true);
            }
            _ => panic!("Expected KeyValue cell"),
        }
    }
}
