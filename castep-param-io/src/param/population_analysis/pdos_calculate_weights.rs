use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

/// This keyword specifies whether or not the weight of the bands in each localized
/// orbital will be calculated for the final ground state of the calculation,
/// in order to allow a partial density of states analysis to be performed.
/// # Default
/// FALSE
/// # Example
/// `PDOS_CALCULATE_WEIGHTS : TRUE`
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
    KeywordDisplay,
)]
#[keyword_display(field="PDOS_CALCULATE_WEIGHTS", from=bool,value=bool)]
pub struct PDOSCalculateWeights(bool);
