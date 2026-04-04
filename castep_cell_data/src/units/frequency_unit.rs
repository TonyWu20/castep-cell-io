use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::FromCellValue;
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;
use serde::{Deserialize, Serialize};

/// Specifies the units in which frequency will be reported.
///
/// Keyword type: String
///
/// Default: FrequencyUnit::Wavenumber
///
/// Example:
/// FREQUENCY_UNIT : hz
#[derive(
    Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(rename = "FREQUENCY_UNIT")]
pub enum FrequencyUnit {
    /// Hartree
    #[serde(alias = "HA", alias = "ha")]
    Hartree,
    /// Millihartree
    #[serde(alias = "MHA", alias = "mha")]
    Millihartree,
    /// Electron Volt
    #[serde(alias = "EV", alias = "ev")]
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
    #[default]
    Wavenumber,
    /// Kelvin
    #[serde(alias = "K", alias = "k")]
    Kelvin,
}

impl FromCellValue for FrequencyUnit {
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
                "unknown FrequencyUnit: {other}"
            ))),
        }
    }
}

impl ToCell for FrequencyUnit {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("FREQUENCY_UNIT", self.to_cell_value())
    }
}

impl ToCellValue for FrequencyUnit {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                FrequencyUnit::Hartree => "ha",
                FrequencyUnit::Millihartree => "mha",
                FrequencyUnit::ElectronVolt => "ev",
                FrequencyUnit::MilliElectronVolt => "mev",
                FrequencyUnit::Rydberg => "ry",
                FrequencyUnit::Millirydberg => "mry",
                FrequencyUnit::KilojoulesPerMole => "kj/mol",
                FrequencyUnit::KilocaloriesPerMole => "kcal/mol",
                FrequencyUnit::Joules => "j",
                FrequencyUnit::Erg => "erg",
                FrequencyUnit::Hertz => "hz",
                FrequencyUnit::Megahertz => "mhz",
                FrequencyUnit::Gigahertz => "ghz",
                FrequencyUnit::Terahertz => "thz",
                FrequencyUnit::Wavenumber => "cm-1",
                FrequencyUnit::Kelvin => "k",
            }
            .to_string(),
        )
    }
}


