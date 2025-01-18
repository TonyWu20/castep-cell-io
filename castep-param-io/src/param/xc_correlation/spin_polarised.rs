use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Default,
    Hash,
    KeywordDisplay,
)]
#[keyword_display(field = "SPIN_POLARISED")]
pub enum SpinPolarised {
    True,
    #[default]
    False,
}

impl From<bool> for SpinPolarised {
    fn from(value: bool) -> Self {
        if value {
            SpinPolarised::True
        } else {
            SpinPolarised::False
        }
    }
}
