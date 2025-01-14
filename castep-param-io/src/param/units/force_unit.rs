use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, Default, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
/// This keyword specifies the units in which force will be reported.
/// # Example
/// `FORCE_UNIT : n`
pub enum ForceUnit {
    HartreePerBohr,
    #[default]
    ElectronVoltsPerAng,
    Newton,
}

impl Display for ForceUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForceUnit::HartreePerBohr => f.write_str("hartree/bohr"),
            ForceUnit::ElectronVoltsPerAng => f.write_str("ev/ang"),
            ForceUnit::Newton => f.write_str("n"),
        }
    }
}

impl KeywordDisplay for ForceUnit {
    fn field(&self) -> String {
        "FORCE_UNIT".to_string()
    }
}
