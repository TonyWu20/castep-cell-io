use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, Default,
)]
pub struct PageExPot(i64);

impl Display for PageExPot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl KeywordDisplay for PageExPot {
    fn field(&self) -> String {
        "NLXC_PAGE_EX_POT".to_string()
    }
}

impl From<i64> for PageExPot {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
