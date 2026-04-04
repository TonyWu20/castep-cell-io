use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult};
use castep_cell_io::query::value_as_u32;
use serde::{Deserialize, Serialize};

/// Controls the number of extra bands at each k-point in addition to the number of occupied bands.
///
/// Keyword type: Integer
///
/// Default: 0
///
/// Example:
/// BS_NEXTRA_BANDS : 12
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "BS_NEXTRA_BANDS")]
pub struct BsNextraBands(pub u32);

impl FromCellValue for BsNextraBands {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for BsNextraBands {
    const KEY_NAME: &'static str = "BS_NEXTRA_BANDS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for BsNextraBands {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("BS_NEXTRA_BANDS", CellValue::UInt(self.0))
    }
}

impl ToCellValue for BsNextraBands {
    fn to_cell_value(&self) -> CellValue {
        CellValue::UInt(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::UInt(12);
        let result = BsNextraBands::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 12);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(BsNextraBands::KEY_NAME, "BS_NEXTRA_BANDS");
    }
}
