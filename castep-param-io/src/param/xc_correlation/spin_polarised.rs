use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default, Hash,
)]
pub struct SpinPolarised(bool);

impl From<bool> for SpinPolarised {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl Display for SpinPolarised {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl KeywordDisplay for SpinPolarised {
    fn field(&self) -> String {
        "SPIN_POLARISED".to_string()
    }
}
