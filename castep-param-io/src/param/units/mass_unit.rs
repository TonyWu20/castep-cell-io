use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default,
)]
/// This keyword specifies the units in which masses will be reported.
/// # Example
/// `MASS_UNIT : kg`
pub enum MassUnit {
    ElectronMass,
    #[default]
    AtomicMassUnit,
    Kilogram,
    Gram,
}

impl Display for MassUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MassUnit::ElectronMass => f.write_str("me"),
            MassUnit::AtomicMassUnit => f.write_str("amu"),
            MassUnit::Kilogram => f.write_str("kg"),
            MassUnit::Gram => f.write_str("g"),
        }
    }
}

impl KeywordDisplay for MassUnit {
    fn field(&self) -> String {
        "MASS_UNIT".to_string()
    }
}
