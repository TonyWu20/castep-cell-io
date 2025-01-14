use std::fmt::{Display, Write};

use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};


#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    KeywordDisplay,
)]
#[keyword_display(field = "CHARGE_UNIT", direct_display = false)]
pub enum ChargeUnit {
    #[default]
    E,
    C,
}

impl Display for ChargeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChargeUnit::E => f.write_char('e'),
            ChargeUnit::C => f.write_char('c'),
        }
    }
}
