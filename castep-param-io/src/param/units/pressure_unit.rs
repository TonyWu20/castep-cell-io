use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// This keyword specifies the units in which pressure will be reported.
/// # Example
/// `PRESSURE_UNIT : atm`
pub enum PressureUnit {
    HartreePerBohr3,
    ElectronVoltsPerAng3,
    Pascal,
    Megapascal,
    Gigapascal,
    Atmosphere,
    Bar,
    Megabar,
}

impl Display for PressureUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PressureUnit::HartreePerBohr3 => f.write_str("hartree/bohr**3"),
            PressureUnit::ElectronVoltsPerAng3 => f.write_str("ev/ang**3"),
            PressureUnit::Pascal => f.write_str("pa"),
            PressureUnit::Megapascal => f.write_str("mpa"),
            PressureUnit::Gigapascal => f.write_str("gpa"),
            PressureUnit::Atmosphere => f.write_str("atm"),
            PressureUnit::Bar => f.write_str("bar"),
            PressureUnit::Megabar => f.write_str("mbar"),
        }
    }
}

impl KeywordDisplay for PressureUnit {
    fn field(&self) -> String {
        "PRESSURE_UNIT".to_string()
    }
}
