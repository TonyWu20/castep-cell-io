use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult};
use castep_cell_io::query::value_as_u32;
use serde::{Deserialize, Serialize};

/// Controls the number of extra bands at each k-point for optics calculations in addition to the number of occupied bands.
///
/// Keyword type: Integer
///
/// Default: 0
///
/// Example:
/// OPTICS_NEXTRA_BANDS : 12
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "OPTICS_NEXTRA_BANDS")]
pub struct OpticsNextraBands(pub u32);

impl FromCellValue for OpticsNextraBands {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for OpticsNextraBands {
    const KEY_NAME: &'static str = "OPTICS_NEXTRA_BANDS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for OpticsNextraBands {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("OPTICS_NEXTRA_BANDS", CellValue::UInt(self.0))
    }
}

impl ToCellValue for OpticsNextraBands {
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
        let result = OpticsNextraBands::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 12);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(OpticsNextraBands::KEY_NAME, "OPTICS_NEXTRA_BANDS");
    }

    #[test]
    fn test_to_cell() {
        let optics_nextra_bands = OpticsNextraBands(12);
        let cell = optics_nextra_bands.to_cell();
        match cell {
            Cell::KeyValue(key, CellValue::UInt(val)) => {
                assert_eq!(key, "OPTICS_NEXTRA_BANDS");
                assert_eq!(val, 12);
            }
            _ => panic!("Expected KeyValue cell"),
        }
    }

    #[test]
    fn test_to_cell_value() {
        let optics_nextra_bands = OpticsNextraBands(12);
        let cell_value = optics_nextra_bands.to_cell_value();
        match cell_value {
            CellValue::UInt(val) => assert_eq!(val, 12),
            _ => panic!("Expected UInt cell value"),
        }
    }

    #[test]
    fn test_default() {
        let optics_nextra_bands = OpticsNextraBands::default();
        assert_eq!(optics_nextra_bands.0, 0);
    }
}
