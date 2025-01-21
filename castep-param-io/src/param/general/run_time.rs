
use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};


/// This keyword specifies the maximum run time for the job, in seconds. If the RUN_TIME is greater than zero, the job will exit cleanly before the specified time has elapsed, leaving as little unused time as possible.
/// If RUN_TIME is less than or equal to zero, no time limit will be imposed on the run.
/// # Default
/// 0
/// # Example
/// `RUN_TIME : 360`
#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    Serialize,
    Deserialize,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    KeywordDisplay,
)]
#[keyword_display(field="RUN_TIME",from=i64,value=i64)]
pub struct RunTime(i64);
