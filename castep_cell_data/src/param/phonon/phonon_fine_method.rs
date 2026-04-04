use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;
use serde::{Deserialize, Serialize};

/// Selects which calculation method to use for phonon calculation on a fine grid.
///
/// Keyword type: String
///
/// Default: PhononFineMethod::None
///
/// Example:
/// PHONON_FINE_METHOD : SUPERCELL
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "PHONON_FINE_METHOD")]
pub enum PhononFineMethod {
    /// No interpolation, direct calculations
    #[serde(alias = "none", alias = "NONE")]
    None,
    /// Use Linear response (density functional perturbation theory) method
    #[serde(alias = "interpolate", alias = "INTERPOLATE")]
    Interpolate,
    /// Use Finite displacement method
    #[serde(alias = "supercell", alias = "SUPERCELL")]
    Supercell,
}

impl FromCellValue for PhononFineMethod {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "none" => Ok(Self::None),
            "interpolate" => Ok(Self::Interpolate),
            "supercell" => Ok(Self::Supercell),
            other => Err(Error::Message(format!("unknown PhononFineMethod: {other}"))),
        }
    }
}

impl FromKeyValue for PhononFineMethod {
    const KEY_NAME: &'static str = "PHONON_FINE_METHOD";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl Default for PhononFineMethod {
    fn default() -> Self {
        Self::None // Default is NONE
    }
}

impl ToCell for PhononFineMethod {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PHONON_FINE_METHOD", self.to_cell_value())
    }
}

impl ToCellValue for PhononFineMethod {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                PhononFineMethod::None => "NONE",
                PhononFineMethod::Interpolate => "INTERPOLATE",
                PhononFineMethod::Supercell => "SUPERCELL",
            }
            .to_string(),
        )
    }
}


