use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default,
)]
/// This keyword specifies the units in which frequency will be reported.
/// # Example
/// `FREQUENCY_UNIT : hz`
pub enum FrequencyUnit {
    Hartree,
    Millihartree,
    ElectronVolt,
    MillielectronVolt,
    Rydberg,
    Millirydberg,
    KilojoulesPerMole,
    KilocaloriesPerMole,
    Joules,
    Erg,
    Hertz,
    Megahertz,
    Gigahertz,
    Terahertz,
    #[default]
    Wavenumber,
    Kelvin,
}

impl Display for FrequencyUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrequencyUnit::Hartree => f.write_str("ha"),
            FrequencyUnit::Millihartree => f.write_str("mha"),
            FrequencyUnit::ElectronVolt => f.write_str("ev"),
            FrequencyUnit::MillielectronVolt => f.write_str("mev"),
            FrequencyUnit::Rydberg => f.write_str("ry"),
            FrequencyUnit::Millirydberg => f.write_str("mry"),
            FrequencyUnit::KilojoulesPerMole => f.write_str("kj/mol"),
            FrequencyUnit::KilocaloriesPerMole => f.write_str("kcal/mol"),
            FrequencyUnit::Joules => f.write_str("j"),
            FrequencyUnit::Erg => f.write_str("erg"),
            FrequencyUnit::Hertz => f.write_str("hz"),
            FrequencyUnit::Megahertz => f.write_str("mhz"),
            FrequencyUnit::Gigahertz => f.write_str("ghz"),
            FrequencyUnit::Terahertz => f.write_str("thz"),
            FrequencyUnit::Wavenumber => f.write_str("cm-1"),
            FrequencyUnit::Kelvin => f.write_str("k"),
        }
    }
}

impl KeywordDisplay for FrequencyUnit {
    fn field(&self) -> String {
        "FREQUENCY_UNIT".to_string()
    }
}
