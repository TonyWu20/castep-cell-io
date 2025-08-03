use castep_cell_serde::{CellValue, ToCellValue};
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

impl ToCellValue for TemperatureUnit {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String("K".to_string())
    }
}
