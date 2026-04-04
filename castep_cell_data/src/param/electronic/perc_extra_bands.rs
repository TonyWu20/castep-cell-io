use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult};
use castep_cell_io::query::value_as_f64;

/// Controls the percentage of extra bands at each k-point in addition to the number of occupied bands.
///
/// Keyword type: Real
///
/// Default: 0.0
///
/// Example:
/// PERC_EXTRA_BANDS : 60.0
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct PercExtraBands(pub f64);

impl FromCellValue for PercExtraBands {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for PercExtraBands {
    const KEY_NAME: &'static str = "PERC_EXTRA_BANDS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for PercExtraBands {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PERC_EXTRA_BANDS", CellValue::Float(self.0))
    }
}

impl ToCellValue for PercExtraBands {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Float(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Float(60.0);
        let result = PercExtraBands::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 60.0);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(PercExtraBands::KEY_NAME, "PERC_EXTRA_BANDS");
    }
}
