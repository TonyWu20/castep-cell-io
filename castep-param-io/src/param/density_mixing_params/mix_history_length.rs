use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

/// This keyword determines the maximum number of densities to store in the
/// history used during the density mixing procedure.
/// # Default
/// 7
/// Example
/// `MIX_HISTORY_LENGTH : 5`
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, KeywordDisplay,
)]
#[keyword_display(field="MIX_HISTORY_LENGTH", from=u32,value=u32, default_value=7)]
pub struct MixHistoryLength(u32);
