use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::FromCellValue;
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;
use serde::{Deserialize, Serialize};

/// Specifies the units in which energies will be reported.
///
/// Keyword type: String
///
/// Default: EnergyUnit::Ev
///
/// Example:
/// ENERGY_UNIT : kcal/mol
#[derive(
    Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(rename = "ENERGY_UNIT")]
pub enum EnergyUnit {
    /// Hartree
    #[serde(alias = "HA", alias = "ha")]
    Hartree,
    /// Millihartree
    #[serde(alias = "MHA", alias = "mha")]
    Millihartree,
    /// Electron Volt
    #[serde(alias = "EV", alias = "ev")]
    #[default]
    ElectronVolt,
    /// Milli-electron Volt
    #[serde(alias = "MEV", alias = "mev")]
    MilliElectronVolt,
    /// Rydberg
    #[serde(alias = "RY", alias = "ry")]
    Rydberg,
    /// Millirydberg
    #[serde(alias = "MRY", alias = "mry")]
    Millirydberg,
    /// Kilojoules per mole
    #[serde(alias = "KJ/MOL", alias = "kj/mol")]
    KilojoulesPerMole,
    /// Kilocalories per mole
    #[serde(alias = "KCAL/MOL", alias = "kcal/mol")]
    KilocaloriesPerMole,
    /// Joules
    #[serde(alias = "J", alias = "j")]
    Joules,
    /// Erg
    #[serde(alias = "ERG", alias = "erg")]
    Erg,
    /// Hertz
    #[serde(alias = "HZ", alias = "hz")]
    Hertz,
    /// Megahertz
    #[serde(alias = "MHZ", alias = "mhz")]
    Megahertz,
    /// Gigahertz
    #[serde(alias = "GHZ", alias = "ghz")]
    Gigahertz,
    /// Terahertz
    #[serde(alias = "THZ", alias = "thz")]
    Terahertz,
    /// Wavenumber
    #[serde(alias = "CM-1", alias = "cm-1")]
    Wavenumber,
    /// Kelvin
    #[serde(alias = "K", alias = "k")]
    Kelvin,
}

impl FromCellValue for EnergyUnit {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "ha" => Ok(Self::Hartree),
            "mha" => Ok(Self::Millihartree),
            "ev" => Ok(Self::ElectronVolt),
            "mev" => Ok(Self::MilliElectronVolt),
            "ry" => Ok(Self::Rydberg),
            "mry" => Ok(Self::Millirydberg),
            "kj/mol" => Ok(Self::KilojoulesPerMole),
            "kcal/mol" => Ok(Self::KilocaloriesPerMole),
            "j" => Ok(Self::Joules),
            "erg" => Ok(Self::Erg),
            "hz" => Ok(Self::Hertz),
            "mhz" => Ok(Self::Megahertz),
            "ghz" => Ok(Self::Gigahertz),
            "thz" => Ok(Self::Terahertz),
            "cm-1" => Ok(Self::Wavenumber),
            "k" => Ok(Self::Kelvin),
            other => Err(Error::Message(format!(
                "unknown EnergyUnit: {other}"
            ))),
        }
    }
}

impl ToCell for EnergyUnit {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("ENERGY_UNIT", self.to_cell_value())
    }
}

impl ToCellValue for EnergyUnit {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                EnergyUnit::Hartree => "ha",
                EnergyUnit::Millihartree => "mha",
                EnergyUnit::ElectronVolt => "ev",
                EnergyUnit::MilliElectronVolt => "mev",
                EnergyUnit::Rydberg => "ry",
                EnergyUnit::Millirydberg => "mry",
                EnergyUnit::KilojoulesPerMole => "kj/mol",
                EnergyUnit::KilocaloriesPerMole => "kcal/mol",
                EnergyUnit::Joules => "j",
                EnergyUnit::Erg => "erg",
                EnergyUnit::Hertz => "hz",
                EnergyUnit::Megahertz => "mhz",
                EnergyUnit::Gigahertz => "ghz",
                EnergyUnit::Terahertz => "thz",
                EnergyUnit::Wavenumber => "cm-1",
                EnergyUnit::Kelvin => "k",
            }
            .to_string(),
        )
    }
}


