use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, Hash, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord,
)]
/// This keyword, if present, will cause the current run to be aborted as if RUN_TIME had been exceeded.
///
/// CASTEP checks the contents of the input file periodically during a run. This allows you to modify certain parameters and also to terminate the run early.
///
/// This keyword is valid only when the input file is reread. It is ignored if it is present at the start of a run.
/// # Example
/// `STOP`
pub struct Stop;
impl Display for Stop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("")
    }
}

impl KeywordDisplay for Stop {
    fn field(&self) -> String {
        "STOP".to_string()
    }
    fn output(&self) -> String {
        self.field()
    }
}
