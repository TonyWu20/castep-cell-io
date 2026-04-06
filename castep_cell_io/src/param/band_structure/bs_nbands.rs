use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult};
use castep_cell_fmt::query::value_as_u32;
use serde::{Deserialize, Serialize};

/// Determines the number of bands at each k-point when performing a band structure calculation.
///
/// Keyword type: Integer
///
/// Default: NBANDS + 5√NBANDS (context-dependent, not directly representable here)
///
/// Example:
/// BS_NBANDS : 64
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "BS_NBANDS")]
pub struct BsNbands(pub u32);

impl FromCellValue for BsNbands {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for BsNbands {
    const KEY_NAME: &'static str = "BS_NBANDS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for BsNbands {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("BS_NBANDS", CellValue::UInt(self.0))
    }
}

impl ToCellValue for BsNbands {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::UInt(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::UInt(64);
        let result = BsNbands::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 64);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(BsNbands::KEY_NAME, "BS_NBANDS");
    }
}
