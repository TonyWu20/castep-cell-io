use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult};
use castep_cell_fmt::query::value_as_u32;
use serde::{Deserialize, Serialize};

/// Determines the maximum number of bands for optics calculations.
///
/// Keyword type: Integer
///
/// Default: Derived from BS_NBANDS
///
/// Example:
/// OPTICS_NBANDS : 64
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "OPTICS_NBANDS")]
pub struct OpticsNbands(pub u32);

impl FromCellValue for OpticsNbands {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for OpticsNbands {
    const KEY_NAME: &'static str = "OPTICS_NBANDS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for OpticsNbands {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("OPTICS_NBANDS", CellValue::UInt(self.0))
    }
}

impl ToCellValue for OpticsNbands {
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
        let result = OpticsNbands::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 64);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(OpticsNbands::KEY_NAME, "OPTICS_NBANDS");
    }

    #[test]
    fn test_to_cell() {
        let optics_nbands = OpticsNbands(64);
        let cell = optics_nbands.to_cell();
        match cell {
            Cell::KeyValue(key, CellValue::UInt(val)) => {
                assert_eq!(key, "OPTICS_NBANDS");
                assert_eq!(val, 64);
            }
            _ => panic!("Expected KeyValue cell"),
        }
    }

    #[test]
    fn test_to_cell_value() {
        let optics_nbands = OpticsNbands(64);
        let cell_value = optics_nbands.to_cell_value();
        match cell_value {
            CellValue::UInt(val) => assert_eq!(val, 64),
            _ => panic!("Expected UInt cell value"),
        }
    }

    #[test]
    fn test_default() {
        let optics_nbands = OpticsNbands::default();
        assert_eq!(optics_nbands.0, 0);
    }
}
