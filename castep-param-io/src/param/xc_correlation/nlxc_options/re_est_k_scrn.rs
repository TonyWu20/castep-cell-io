use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, Default,
)]
pub struct ReEstKScrn(bool);

impl From<bool> for ReEstKScrn {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl Display for ReEstKScrn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl KeywordDisplay for ReEstKScrn {
    fn field(&self) -> String {
        "NLXC_RE_EST_K_SCRN".to_string()
    }
}
