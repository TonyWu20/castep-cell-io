use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult};
use castep_cell_fmt::query::value_as_u32;

/// Controls paging of exchange potential in non-local exchange-correlation.
///
/// Keyword type: Integer
///
/// Default: 0
///
/// Aliases: PAGE_EX_POT
///
/// Example:
/// NLXC_PAGE_EX_POT : 0
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct NlxcPageExPot(pub u32);

impl FromCellValue for NlxcPageExPot {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for NlxcPageExPot {
    const KEY_NAME: &'static str = "PAGE_EX_POT";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for NlxcPageExPot {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("NLXC_PAGE_EX_POT", CellValue::UInt(self.0))
    }
}

impl ToCellValue for NlxcPageExPot {
    fn to_cell_value(&self) -> CellValue {
        CellValue::UInt(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value_zero() {
        let val = CellValue::UInt(0);
        assert_eq!(NlxcPageExPot::from_cell_value(&val).unwrap().0, 0);
    }

    #[test]
    fn test_from_cell_value_nonzero() {
        let val = CellValue::UInt(42);
        assert_eq!(NlxcPageExPot::from_cell_value(&val).unwrap().0, 42);
    }

    #[test]
    fn test_default() {
        assert_eq!(NlxcPageExPot::default().0, 0);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(NlxcPageExPot::KEY_NAME, "PAGE_EX_POT");
    }

    #[test]
    fn test_to_cell() {
        let pot = NlxcPageExPot(42);
        match pot.to_cell() {
            Cell::KeyValue(key, CellValue::UInt(val)) => {
                assert_eq!(key, "NLXC_PAGE_EX_POT");
                assert_eq!(val, 42);
            }
            _ => panic!("Expected KeyValue cell"),
        }
    }
}
