use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult};
use castep_cell_fmt::query::value_as_bool;

/// Controls whether k-point reflection symmetry is used in non-local exchange-correlation.
///
/// Keyword type: Logical
///
/// Default: TRUE
///
/// Aliases: EXCHANGE_REFLECT_KPTS
///
/// Example:
/// NLXC_EXCHANGE_REFLECT_KPTS : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NlxcExchangeReflectKpts(pub bool);

impl Default for NlxcExchangeReflectKpts {
    fn default() -> Self {
        Self(true)
    }
}

impl FromCellValue for NlxcExchangeReflectKpts {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl FromKeyValue for NlxcExchangeReflectKpts {
    const KEY_NAME: &'static str = "EXCHANGE_REFLECT_KPTS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for NlxcExchangeReflectKpts {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("NLXC_EXCHANGE_REFLECT_KPTS", CellValue::Bool(self.0))
    }
}

impl ToCellValue for NlxcExchangeReflectKpts {
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
        assert_eq!(NlxcExchangeReflectKpts::from_cell_value(&val).unwrap().0, true);
    }

    #[test]
    fn test_from_cell_value_false() {
        let val = CellValue::Bool(false);
        assert_eq!(NlxcExchangeReflectKpts::from_cell_value(&val).unwrap().0, false);
    }

    #[test]
    fn test_default() {
        assert_eq!(NlxcExchangeReflectKpts::default().0, true);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(NlxcExchangeReflectKpts::KEY_NAME, "EXCHANGE_REFLECT_KPTS");
    }

    #[test]
    fn test_to_cell() {
        let kpt = NlxcExchangeReflectKpts(true);
        match kpt.to_cell() {
            Cell::KeyValue(key, CellValue::Bool(val)) => {
                assert_eq!(key, "NLXC_EXCHANGE_REFLECT_KPTS");
                assert_eq!(val, true);
            }
            _ => panic!("Expected KeyValue cell"),
        }
    }
}
