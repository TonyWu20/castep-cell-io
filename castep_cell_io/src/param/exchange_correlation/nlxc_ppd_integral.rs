use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult};
use castep_cell_fmt::query::value_as_bool;

/// Controls whether plane-wave density integral is used in non-local exchange-correlation.
///
/// Keyword type: Logical
///
/// Default: FALSE
///
/// Aliases: PPD_INTEGRAL
///
/// Example:
/// NLXC_PPD_INTEGRAL : FALSE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct NlxcPpdIntegral(pub bool);

impl FromCellValue for NlxcPpdIntegral {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl FromKeyValue for NlxcPpdIntegral {
    const KEY_NAME: &'static str = "PPD_INTEGRAL";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for NlxcPpdIntegral {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("NLXC_PPD_INTEGRAL", CellValue::Bool(self.0))
    }
}

impl ToCellValue for NlxcPpdIntegral {
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
        assert_eq!(NlxcPpdIntegral::from_cell_value(&val).unwrap().0, true);
    }

    #[test]
    fn test_from_cell_value_false() {
        let val = CellValue::Bool(false);
        assert_eq!(NlxcPpdIntegral::from_cell_value(&val).unwrap().0, false);
    }

    #[test]
    fn test_default() {
        assert_eq!(NlxcPpdIntegral::default().0, false);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(NlxcPpdIntegral::KEY_NAME, "PPD_INTEGRAL");
    }

    #[test]
    fn test_to_cell() {
        let ppd = NlxcPpdIntegral(true);
        match ppd.to_cell() {
            Cell::KeyValue(key, CellValue::Bool(val)) => {
                assert_eq!(key, "NLXC_PPD_INTEGRAL");
                assert_eq!(val, true);
            }
            _ => panic!("Expected KeyValue cell"),
        }
    }
}
