use castep_cell_io::{CellValue, ToCellValue};
use castep_cell_io::parse::FromCellValue;
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;
use serde::{Deserialize, Serialize};

/// Specifies the units for the electric field vector in the EXTERNAL_EFIELD block.
#[derive(
    Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(rename = "TEMPERATURE_UNIT")] // Name for serde, though likely used via variant names
pub enum TemperatureUnit {
    /// The default unit: K
    #[serde(alias = "k")] // CASTEP notation for eV/Å/electron
    #[default]
    K,
}

impl FromCellValue for TemperatureUnit {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "k" => Ok(Self::K),
            other => Err(Error::Message(format!(
                "unknown TemperatureUnit: {other}"
            ))),
        }
    }
}

impl ToCellValue for TemperatureUnit {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String("K".to_string())
    }
}
