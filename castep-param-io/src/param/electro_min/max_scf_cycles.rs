use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};
/// This keyword determines the maximum number of SCF cycles performed in an electronic minimization. The electronic minimization will end after this many cycles, regardless of whether the convergence criteria have been met.
/// # Default
/// 30
/// # Example
/// `MAX_SCF_CYCLES : 20`
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, KeywordDisplay,
)]
#[keyword_display(field="MAX_SCF_CYCLES", from=u32,value= u32)]
pub struct MaxScfCycles(u32);
