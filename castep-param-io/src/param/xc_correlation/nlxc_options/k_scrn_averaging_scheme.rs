use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub enum KScrnAveragingScheme {
    #[default]
    AveDen,
    SwaDen,
    SwaEsx,
}

impl Display for KScrnAveragingScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KScrnAveragingScheme::AveDen => f.write_str("AVE_DEN"),
            KScrnAveragingScheme::SwaDen => f.write_str("SWA_DEN"),
            KScrnAveragingScheme::SwaEsx => f.write_str("SWA_ESX"),
        }
    }
}

impl KeywordDisplay for KScrnAveragingScheme {
    fn field(&self) -> String {
        "NLXC_K_SCRN_AVERAGING_SCHEME".to_string()
    }
}
