use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult};
use castep_cell_fmt::query::value_as_f64;
use serde::{Deserialize, Serialize};

/// Controls the percentage of extra bands at each k-point in addition to the number of occupied bands.
///
/// Keyword type: Real
///
/// Default: 0.0
///
/// Example:
/// BS_PERC_EXTRA_BANDS : 60.0
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "BS_PERC_EXTRA_BANDS")]
pub struct BsPercExtraBands(pub f64);

impl FromCellValue for BsPercExtraBands {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for BsPercExtraBands {
    const KEY_NAME: &'static str = "BS_PERC_EXTRA_BANDS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for BsPercExtraBands {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("BS_PERC_EXTRA_BANDS", CellValue::Float(self.0))
    }
}

impl ToCellValue for BsPercExtraBands {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Float(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Float(60.0);
        let result = BsPercExtraBands::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 60.0);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(BsPercExtraBands::KEY_NAME, "BS_PERC_EXTRA_BANDS");
    }
}
