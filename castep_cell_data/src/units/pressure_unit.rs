use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::FromCellValue;
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;
use serde::{Deserialize, Serialize};

/// Specifies the units in which pressure will be reported.
///
/// Keyword type: String
///
/// Default: gpa
///
/// Example:
/// PRESSURE_UNIT : atm
#[derive(
    Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(rename = "PRESSURE_UNIT")] // Ensures correct key name during serde
pub enum PressureUnit {
    /// Hartree per Bohr^3
    #[serde(alias = "HARTREE/BOHR**3", alias = "hartree/bohr**3")]
    HartreePerBohr3,
    /// Electron Volts per Å^3
    #[serde(alias = "EV/ANG**3", alias = "ev/ang**3")]
    EvPerAng3,
    /// Pascal
    #[serde(alias = "PA", alias = "pa")]
    Pascal,
    /// Megapascal
    #[serde(alias = "MPA", alias = "mpa")]
    MegaPascal,
    /// Gigapascal (Default)
    #[serde(alias = "GPA", alias = "gpa")]
    #[default]
    GigaPascal,
    /// Atmosphere
    #[serde(alias = "ATM", alias = "atm")]
    Atmosphere,
    /// Bar
    #[serde(alias = "BAR", alias = "bar")]
    Bar,
    /// Megabar
    #[serde(alias = "MBAR", alias = "mbar")]
    MegaBar,
}

impl PressureUnit {
    /// Returns the default unit.
    pub const fn default_unit() -> Self {
        Self::GigaPascal // Default is gpa
    }
}

impl FromCellValue for PressureUnit {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "hartree/bohr**3" => Ok(Self::HartreePerBohr3),
            "ev/ang**3" => Ok(Self::EvPerAng3),
            "pa" => Ok(Self::Pascal),
            "mpa" => Ok(Self::MegaPascal),
            "gpa" => Ok(Self::GigaPascal),
            "atm" => Ok(Self::Atmosphere),
            "bar" => Ok(Self::Bar),
            "mbar" => Ok(Self::MegaBar),
            other => Err(Error::Message(format!(
                "unknown PressureUnit: {other}"
            ))),
        }
    }
}

// Implement ToCell for PressureUnit to enable serialization via your custom backend
impl ToCell for PressureUnit {
    fn to_cell(&self) -> Cell {
        // Create a KeyValue Cell with the name "PRESSURE_UNIT" and the unit string as the value.
        Cell::KeyValue("PRESSURE_UNIT", self.to_cell_value())
    }
}

// Implement ToCellValue for PressureUnit.
impl ToCellValue for PressureUnit {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                PressureUnit::HartreePerBohr3 => "hartree/bohr**3",
                PressureUnit::EvPerAng3 => "ev/ang**3",
                PressureUnit::Pascal => "pa",
                PressureUnit::MegaPascal => "mpa",
                PressureUnit::GigaPascal => "gpa",
                PressureUnit::Atmosphere => "atm",
                PressureUnit::Bar => "bar",
                PressureUnit::MegaBar => "mbar",
            }
            .to_string(), // Convert &str to String for CellValue::String
        )
    }
}


