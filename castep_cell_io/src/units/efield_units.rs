use castep_cell_fmt::{CellValue, ToCellValue};
use castep_cell_fmt::parse::FromCellValue;
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_str;
use serde::{Deserialize, Serialize};

/// Specifies the units for the electric field vector in the EXTERNAL_EFIELD block.
#[derive(
    Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(rename = "EFIELD_UNIT")] // Name for serde, though likely used via variant names
pub enum EFieldUnit {
    /// The default unit: eV/Å/electron
    #[serde(rename = "ev/ang/e", alias = "EV/ANG/E")] // CASTEP notation for eV/Å/electron
    #[default]
    EvPerAngPerE,
    /// Hartree per Bohr per electron
    #[serde(rename = "hartree/bohr/e", alias = "HARTREE/BOHR/E")]
    HartreePerBohrPerE,
    // Add other units if they become valid/used in CASTEP for this context
    #[serde(rename = "N/C", alias = "n/c")]
    NewtonPerCharge,
}

// Implement ToCellValue for EFieldUnit to allow serialization via your backend.
impl FromCellValue for EFieldUnit {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "ev/ang/e" => Ok(Self::EvPerAngPerE),
            "hartree/bohr/e" => Ok(Self::HartreePerBohrPerE),
            "n/c" => Ok(Self::NewtonPerCharge),
            other => Err(Error::Message(format!(
                "unknown EFieldUnit: {other}"
            ))),
        }
    }
}

impl ToCellValue for EFieldUnit {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::String(
            match self {
                EFieldUnit::EvPerAngPerE => "ev/ang/e",
                EFieldUnit::HartreePerBohrPerE => "hartree/bohr/e",
                EFieldUnit::NewtonPerCharge => "n/c", // Add arms for other variants
            }
            .to_string(), // Convert &str to String for CellValue::String
        )
    }
}


