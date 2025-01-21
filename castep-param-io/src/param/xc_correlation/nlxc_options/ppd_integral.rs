use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, Default,
)]
pub struct PPDIntegral(bool);

impl Display for PPDIntegral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl KeywordDisplay for PPDIntegral {
    fn field(&self) -> String {
        "NLXC_PPD_INTEGRAL".to_string()
    }
}

impl From<bool> for PPDIntegral {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
