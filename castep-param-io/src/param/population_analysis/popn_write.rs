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
    Hash,
    Default,
    Serialize,
    Deserialize,
    KeywordDisplay,
)]
#[keyword_display(field = "POPN_WRITE")]
/// This keyword specifies the verbosity of reporting of population analysis results.
/// The available values are:
/// | Value | Meaning |
/// |-------|----------|
/// |NONE    | No output |
/// | MINIMAL | Summary only |
/// | SUMMARY | Same as MINIMAL |
/// | ENHANCED | Summary and orbital-resolved PDOS components |
/// | VERBOSE | As ENHANCED and S and T matrices |
/// # Default
/// ENHANCED
/// # Example
/// `POPN_WRITE : SUMMARY`
pub enum PopnWrite {
    None,
    Minimal,
    Summary,
    #[default]
    Enhanced,
    Verbose,
}
