use castep_param_derive::KeywordDisplayStruct;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::InvLengthUnit;

/// This keyword determines the maximum size of the g-vectors included in the fine grid.
/// The fine grid is seDefault
/// -1 a0-1 - this results in the fine and standard grids being identical
/// # Example
/// FINE_GMAX : 20 1/angt up such that all g-vectors with |g| less than or equal to FINE_GMAX are included.
#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Builder, KeywordDisplayStruct,
)]
#[builder(setter(into), default)]
#[keyword_display(field = "FINE_GMAX", display_format="{:20.15} {}", default_value=-1.0, from=f64)]
pub struct FineGMax {
    pub max: f64,
    #[keyword_display(is_option = true)]
    pub unit: Option<InvLengthUnit>,
}
