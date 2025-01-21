use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default,
)]
/// This keyword specifies the units in which lengths will be reported.
/// # Example
/// `LENGTH_UNIT : bohr`
pub enum LengthUnit {
    Bohr,
    BohrA0,
    Meter,
    Centimeter,
    Nanometer,
    #[default]
    Ang,
}

impl Display for LengthUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LengthUnit::Bohr => f.write_str("bohr"),
            LengthUnit::BohrA0 => f.write_str("a0"),
            LengthUnit::Meter => f.write_str("m"),
            LengthUnit::Centimeter => f.write_str("cm"),
            LengthUnit::Nanometer => f.write_str("nm"),
            LengthUnit::Ang => f.write_str("ang"),
        }
    }
}

impl KeywordDisplay for LengthUnit {
    fn field(&self) -> String {
        "LENGTH_UNIT".to_string()
    }
}
