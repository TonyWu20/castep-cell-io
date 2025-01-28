use std::fmt::Display;
use pest::Parser;
use from_pest::FromPest;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default,
)]
/// This keyword specifies the units in which volume will be reported.
/// # Example
/// ` VOLUME_UNIT : nm**3`
pub enum VolumeUnit {
    Bohr3,
    Meter3,
    Centimeter3,
    Nanometer3,
    #[default]
    Ang3,
}

impl Display for VolumeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VolumeUnit::Bohr3 => f.write_str("bohr**3"),
            VolumeUnit::Meter3 => f.write_str("m**3"),
            VolumeUnit::Centimeter3 => f.write_str("cm**3"),
            VolumeUnit::Nanometer3 => f.write_str("nm**3"),
            VolumeUnit::Ang3 => f.write_str("ang**3"),
        }
    }
}

impl KeywordDisplay for VolumeUnit {
    fn field(&self) -> String {
        "VOLUME_UNIT".to_string()
    }
}
