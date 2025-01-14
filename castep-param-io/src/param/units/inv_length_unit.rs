use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default,
)]
/// This keyword specifies the units in which inverse length will be reported.
/// # Example
/// `INV_LENGTH_UNIT : 1/nm`
pub enum InvLengthUnit {
    Bohr,
    Meter,
    Nanometer,
    #[default]
    Ang,
}

impl Display for InvLengthUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvLengthUnit::Bohr => f.write_str("1/"),
            InvLengthUnit::Meter => f.write_str("1/m"),
            InvLengthUnit::Nanometer => f.write_str("1/nm"),
            InvLengthUnit::Ang => f.write_str("1/ang"),
        }
    }
}

impl KeywordDisplay for InvLengthUnit {
    fn field(&self) -> String {
        "INV_LENGTH_UNIT".to_string()
    }
}
