use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

use super::ElectronicMinimizer;

/// This keyword determines the maximum number of conjugate gradient steps in an SCF cycle.
/// # Default
/// The default depends on the value of `ELECTRONIC_MINIMIZER`:
/// `SD` then `MAX_CG_STEPS` : 0
/// `CG` then `MAX_CG_STEPS` : 4
/// `RMM`/`DIIS` then `MAX_CG_STEPS` : 2
/// If `ELECTRONIC_MINIMIZER` is not defined, the default is 4.
/// # Example
/// `MAX_CG_STEPS : 5`
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, KeywordDisplay,
)]
#[keyword_display(field="MAX_CG_STEPS", from=u32,value=u32, default_value=4)]
pub struct MaxCgSteps(u32);

impl MaxCgSteps {
    pub fn default_value(electronic_minimizer: ElectronicMinimizer) -> Self {
        match electronic_minimizer {
            ElectronicMinimizer::SD => Self(0),
            ElectronicMinimizer::CG => Self(4),
        }
    }
}
