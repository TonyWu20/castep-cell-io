use std::fmt::Display;

use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};


/// This keyword controls whether or not different wavefunctions are used for different spins.
/// # Default
/// `FALSE`
/// # Example
/// `SPIN_POLARIZED : TRUE`
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    KeywordDisplay,
)]
#[keyword_display(field = "SPIN_POLARIZED", direct_display = false)]
pub enum SpinPolarised {
    True,
    #[default]
    False,
}

impl Display for SpinPolarised {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpinPolarised::True => f.write_str("true"),
            SpinPolarised::False => f.write_str("false"),
        }
    }
}

impl From<bool> for SpinPolarised {
    fn from(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}
