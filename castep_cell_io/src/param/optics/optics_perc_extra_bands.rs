use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult};
use castep_cell_fmt::query::value_as_f64;
use serde::{Deserialize, Serialize};

/// Controls the percentage of extra bands at each k-point for optics calculations in addition to the number of occupied bands.
///
/// Keyword type: Real
///
/// Default: 0.0
///
/// Example:
/// OPTICS_PERC_EXTRA_BANDS : 60.0
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "OPTICS_PERC_EXTRA_BANDS")]
pub struct OpticsPercExtraBands(pub f64);

impl FromCellValue for OpticsPercExtraBands {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for OpticsPercExtraBands {
    const KEY_NAME: &'static str = "OPTICS_PERC_EXTRA_BANDS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for OpticsPercExtraBands {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("OPTICS_PERC_EXTRA_BANDS", CellValue::Float(self.0))
    }
}

impl ToCellValue for OpticsPercExtraBands {
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
        let result = OpticsPercExtraBands::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 60.0);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(OpticsPercExtraBands::KEY_NAME, "OPTICS_PERC_EXTRA_BANDS");
    }

    #[test]
    fn test_to_cell() {
        let optics_perc_extra_bands = OpticsPercExtraBands(60.0);
        let cell = optics_perc_extra_bands.to_cell();
        match cell {
            Cell::KeyValue(key, CellValue::Float(val)) => {
                assert_eq!(key, "OPTICS_PERC_EXTRA_BANDS");
                assert_eq!(val, 60.0);
            }
            _ => panic!("Expected KeyValue cell"),
        }
    }

    #[test]
    fn test_to_cell_value() {
        let optics_perc_extra_bands = OpticsPercExtraBands(60.0);
        let cell_value = optics_perc_extra_bands.to_cell_value();
        match cell_value {
            CellValue::Float(val) => assert_eq!(val, 60.0),
            _ => panic!("Expected Float cell value"),
        }
    }

    #[test]
    fn test_default() {
        let optics_perc_extra_bands = OpticsPercExtraBands::default();
        assert_eq!(optics_perc_extra_bands.0, 0.0);
    }
}
