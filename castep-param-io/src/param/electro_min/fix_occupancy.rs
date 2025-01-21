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
#[keyword_display(field="FIX_OCCUPANCY",from=bool,value=bool)]
/// This keyword specifies whether or not the occupancies of the bands should be
/// fixed, that is, if the system should be treated as a zero temperature insulator or a metal.
/// # Default
/// FALSE
/// # Example
/// `FIX_OCCUPANCY : TRUE`
pub struct FixOccupancy(bool);
