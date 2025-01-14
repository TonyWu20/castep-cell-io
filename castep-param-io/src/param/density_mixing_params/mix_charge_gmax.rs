use castep_param_derive::KeywordDisplayStruct;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::InvLengthUnit;

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, KeywordDisplayStruct, Builder,
)]
#[keyword_display(field = "MIX_CHARGE_GMAX", display_format = "{:20.15} {}", from=f64, default_value=1.5)]
#[builder(setter(strip_option), default)]
/// This keyword determines the maximum g-vector at which the charge density is mixed
/// in the density mixing procedure.
/// # Default
/// 1.5 Å-1
/// # Example
/// MIX_CHARGE_GMAX : 0.89 1/ang
pub struct MixChargeGmax {
    pub gmax: f64,
    #[keyword_display(is_option = true)]
    pub unit: Option<InvLengthUnit>,
}
