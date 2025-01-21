use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, KeywordDisplay, Hash,
)]
#[keyword_display(field="NUM_OCC_CYCLES", from=u32, value=u32, default_value=6)]
/// This keyword controls the number of occupancy minimization cycles performed
/// for each electronic minimization step during an EDFT run.
/// # Note
/// This parameter is used only if FIX_OCCUPANCY : FALSE or ELEC_TEMP is greater than 0 and METALS_METHOD : EDFT.
/// # Default
/// 6
/// # Example
/// `NUM_OCC_CYCLES : 10`
pub struct NumOccCycles(u32);
