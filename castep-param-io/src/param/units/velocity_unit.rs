use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default,
)]
/// This keyword specifies the units in which velocity will be reported.
/// # Example
/// `VELOCITY_UNIT : bohr/fs`
pub enum VelocityUnit {
    AtomicUnitOfVelocity,
    #[default]
    AngPerPs,
    AngPerFs,
    BohrPerPs,
    BohrPerFs,
    MetersPerSecond,
}

impl Display for VelocityUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VelocityUnit::AtomicUnitOfVelocity => f.write_str("auv"),
            VelocityUnit::AngPerPs => f.write_str("ang/ps"),
            VelocityUnit::AngPerFs => f.write_str("ang/fs"),
            VelocityUnit::BohrPerPs => f.write_str("bohr/ps"),
            VelocityUnit::BohrPerFs => f.write_str("bohr/fs"),
            VelocityUnit::MetersPerSecond => f.write_str("m/s"),
        }
    }
}

impl KeywordDisplay for VelocityUnit {
    fn field(&self) -> String {
        "VELOCITY_UNIT".to_string()
    }
}
