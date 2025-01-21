use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default,
)]
/// This keyword specifies the units in which force constants will be reported.
/// # Example
/// `FORCE_CONSTANT_UNIT : n/m`
pub enum ForceConstantUnit {
    HartreePerBohr2,
    #[default]
    ElectronVoltsPerAng2,
    NewtonPerMeter,
    DynesPerCentimeter,
}

impl Display for ForceConstantUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForceConstantUnit::HartreePerBohr2 => f.write_str("hartree/bohr**2"),
            ForceConstantUnit::ElectronVoltsPerAng2 => f.write_str("ev/ang**2"),
            ForceConstantUnit::NewtonPerMeter => f.write_str("n/m"),
            ForceConstantUnit::DynesPerCentimeter => f.write_str("dyne/cm"),
        }
    }
}

impl KeywordDisplay for ForceConstantUnit {
    fn field(&self) -> String {
        "FORCE_CONSTANT_UNIT".to_string()
    }
}
