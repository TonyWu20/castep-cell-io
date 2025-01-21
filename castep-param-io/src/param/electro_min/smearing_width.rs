use castep_param_derive::KeywordDisplayStruct;
use serde::{Deserialize, Serialize};

use crate::param::EnergyUnit;

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, KeywordDisplayStruct,
)]
#[keyword_display(field = "SMEARING_WIDTH", from=f64, default_value=0.2, display_format="{:20.15} {}")]
/// This keyword determines the width of the Fermi-surface smearing if the system
/// is being treated as a metal.
/// # Note
/// This parameter is used only if FIX_OCCUPANCY : FALSE.
/// # Default
/// 0.2 eV
/// # Example
/// SMEARING_WIDTH : 0.1 eV
pub struct SmearingWidth {
    pub width: f64,
    #[keyword_display(is_option = true)]
    pub unit: Option<EnergyUnit>,
}
