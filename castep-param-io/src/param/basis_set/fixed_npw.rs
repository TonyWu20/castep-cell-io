
use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};


/// This keyword determines whether a fixed number of plane waves (fixed size
/// basis : TRUE) or a fixed plane wave cutoff energy
/// (fixed quality basis : FALSE) will be used when doing a variable cell
/// calculation.
/// This setting affects geometry optimization with variable cell parameters
/// and molecular dynamics with NPT or NPH ensembles.
/// # Note
/// You should turn off finite basis set correction when using a
/// fixed size basis (see FINITE_BASIS_CORR).
/// # Default
/// `FALSE`
/// # Example
/// `FIXED_NPW : TRUE`
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    KeywordDisplay,
)]
#[keyword_display(field="FIXED_NPW", from=bool,value=bool)]
pub struct FixedNPW(bool);
