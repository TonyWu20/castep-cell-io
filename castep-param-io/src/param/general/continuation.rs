use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Hash, Serialize, Deserialize, KeywordDisplay, PartialEq, Eq, PartialOrd, Ord,
)]
#[keyword_display(specified_fields = true)]
pub enum ContinueReuse {
    #[keyword_display(field = "CONTINUATION")]
    Continuation(Continuation),
    #[keyword_display(field = "REUSE")]
    Reuse(Reuse),
}

impl Default for ContinueReuse {
    fn default() -> Self {
        Self::Continuation(Continuation::Default)
    }
}

impl From<Continuation> for ContinueReuse {
    fn from(value: Continuation) -> Self {
        ContinueReuse::Continuation(value)
    }
}

impl From<Reuse> for ContinueReuse {
    fn from(value: Reuse) -> Self {
        ContinueReuse::Reuse(value)
    }
}

#[derive(
    Debug,
    Clone,
    Default,
    Hash,
    Serialize,
    Deserialize,
    KeywordDisplay,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
#[keyword_display(field = "CONTINUATION")]
pub enum Continuation {
    #[default]
    Default,
    File(String),
}

#[derive(
    Debug,
    Clone,
    Default,
    Hash,
    Serialize,
    Deserialize,
    KeywordDisplay,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
#[keyword_display(field = "REUSE")]
pub enum Reuse {
    #[default]
    Default,
    File(String),
}
