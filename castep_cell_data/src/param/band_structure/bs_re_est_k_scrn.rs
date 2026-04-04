use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult};
use castep_cell_io::query::value_as_bool;
use serde::{Deserialize, Serialize};

/// Determines whether or not to update the estimate of the Thomas-Fermi screening length.
///
/// Keyword type: Logical
///
/// Default: false
///
/// Example:
/// BS_RE_EST_K_SCRN : TRUE
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "BS_RE_EST_K_SCRN")]
pub struct BsReEstKScrn(pub bool);

impl FromCellValue for BsReEstKScrn {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl FromKeyValue for BsReEstKScrn {
    const KEY_NAME: &'static str = "BS_RE_EST_K_SCRN";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for BsReEstKScrn {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("BS_RE_EST_K_SCRN", CellValue::Bool(self.0))
    }
}

impl ToCellValue for BsReEstKScrn {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Bool(true);
        let result = BsReEstKScrn::from_cell_value(&val).unwrap();
        assert!(result.0);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(BsReEstKScrn::KEY_NAME, "BS_RE_EST_K_SCRN");
    }
}
