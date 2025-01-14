use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, Default,
)]
pub struct ImposeTrs(bool);

impl From<bool> for ImposeTrs {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl Display for ImposeTrs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl KeywordDisplay for ImposeTrs {
    fn field(&self) -> String {
        "NLXC_IMPOSE_TRS".to_string()
    }
}
