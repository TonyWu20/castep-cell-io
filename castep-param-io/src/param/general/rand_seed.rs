
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
#[keyword_display(field="RAND_SEED",from=i64,value=i64)]
pub struct RandSeed(i64);
