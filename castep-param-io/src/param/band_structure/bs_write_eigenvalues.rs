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
    Default,
    Serialize,
    Deserialize,
    KeywordDisplay,
)]
#[keyword_display(field="BS_WRITE_EIGENVALUES", from=bool,value=bool)]
pub struct BSWriteEigenvalues(bool);
