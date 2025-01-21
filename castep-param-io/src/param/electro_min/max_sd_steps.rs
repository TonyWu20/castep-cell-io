use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

use super::ElectronicMinimizer;
/// This keyword determines the maximum number of steepest descent steps in an SCF cycle.
/// # Default
/// The default depends on the value of ELECTRONIC_MINIMIZER:
/// SD then MAX_SD_STEPS : 10
/// CG then MAX_SD_STEPS : 1
/// RMM/DIIS then MAX_SD_STEPS : 1
/// If ELECTRONIC_MINIMIZER is not defined, the default is 1.
/// # Example
/// MAX_SD_STEPS : 5
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, KeywordDisplay,
)]
#[keyword_display(field="MAX_SD_STEPS", from=u32,value= u32, default_value=1)]
pub struct MaxSdSteps(u32);

impl MaxSdSteps {
    fn default_value(electronic_minimizer: ElectronicMinimizer) -> Self {
        match electronic_minimizer {
            ElectronicMinimizer::SD => Self(10),
            ElectronicMinimizer::CG => Self(1),
        }
    }
}
