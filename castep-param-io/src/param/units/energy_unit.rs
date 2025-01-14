use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;
#[derive(
    Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default,
)]
/// This keyword specifies the units in which energies will be reported.
/// # Example
/// `ENERGY_UNIT : kcal/mol`
pub enum EnergyUnit {
    Hartree,
    Millihartree,
    #[default]
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
    Wavenumber,
    Kelvin,
}
impl Display for EnergyUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnergyUnit::Hartree => f.write_str("ha"),
            EnergyUnit::Millihartree => f.write_str("mha"),
            EnergyUnit::ElectronVolt => f.write_str("eV"),
            EnergyUnit::MillielectronVolt => f.write_str("meV"),
            EnergyUnit::Rydberg => f.write_str("ry"),
            EnergyUnit::Millirydberg => f.write_str("mry"),
            EnergyUnit::KilojoulesPerMole => f.write_str("kj/mol"),
            EnergyUnit::KilocaloriesPerMole => f.write_str("kcal/mol"),
            EnergyUnit::Joules => f.write_str("j"),
            EnergyUnit::Erg => f.write_str("erg"),
            EnergyUnit::Hertz => f.write_str("hz"),
            EnergyUnit::Megahertz => f.write_str("mhz"),
            EnergyUnit::Gigahertz => f.write_str("ghz"),
            EnergyUnit::Terahertz => f.write_str("thz"),
            EnergyUnit::Wavenumber => f.write_str("cm-1"),
            EnergyUnit::Kelvin => f.write_str("k"),
        }
    }
}

impl KeywordDisplay for EnergyUnit {
    fn field(&self) -> String {
        "ENERGY_UNIT".to_string()
    }
}
