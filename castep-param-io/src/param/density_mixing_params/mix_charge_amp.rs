use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, KeywordDisplay)]
#[keyword_display(field="MIX_CHARGE_AMP", from=f64, value=f64, display_format="{:20.15}")]
/// This keyword determines the mixing amplitude for the charge density in the density mixing procedure.
/// # Default
/// 0.8
/// # Example
/// MIX_CHARGE_AMP : 0.5
pub struct MixChargeAmp(f64);
