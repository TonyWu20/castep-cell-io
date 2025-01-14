use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

use super::FixOccupancy;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    Default,
    KeywordDisplay,
)]
#[keyword_display(field = "METALS_METHOD")]
pub enum MetalsMethod {
    DM,
    #[default]
    EDFT,
}

impl MetalsMethod {
    pub fn default_value(fix_occupancy: FixOccupancy) -> Self {
        if !fix_occupancy.value() {
            Self::DM
        } else {
            Self::EDFT
        }
    }
}
