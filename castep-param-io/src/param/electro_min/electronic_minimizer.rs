use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    KeywordDisplay,
)]
#[keyword_display(field = "ELECTRONIC_MINIMIZER")]
/// This keyword controls the method used to minimize electronic states.
/// Available options are:
/// - SD - minimizer takes up to 10 SD steps.
/// - CG - minimizer takes one SD step followed by up to 4 CG steps.
/// The default values for the number of steps can be overwritten using the MAX_SD_STEPS and MAX_CG_STEPS keywords.
/// # Default
/// `CG`
/// # Example
/// `ELECTRONIC_MINIMIZER : SD`
pub enum ElectronicMinimizer {
    SD,
    #[default]
    CG,
}
