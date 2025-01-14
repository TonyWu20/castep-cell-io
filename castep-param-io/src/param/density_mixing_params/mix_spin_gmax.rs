use castep_param_derive::KeywordDisplayStruct;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::InvLengthUnit;

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, KeywordDisplayStruct, Builder,
)]
#[keyword_display(field = "MIX_SPIN_GMAX", display_format = "{:20.15} {}", from=f64, default_value=1.5)]
/// This keyword determines the maximum g-vector at which the spin density is
/// mixed in the density mixing procedure.
/// # Default
/// 1.5 Å-1
/// # Example
/// `MIX_SPIN_GMAX : 0.89 1/ang`
pub struct MixSpinGmax {
    pub gmax: f64,
    #[keyword_display(is_option = true)]
    pub unit: Option<InvLengthUnit>,
}
