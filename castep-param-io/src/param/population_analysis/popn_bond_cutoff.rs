use castep_param_derive::KeywordDisplayStruct;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::LengthUnit;

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, KeywordDisplayStruct, Serialize, Deserialize, Builder,
)]
#[keyword_display(field = "POPN_BOND_CUTOFF", display_format = "{:20.15} {}", from=f64, default_value=3.0)]
/// This keyword controls the maximum distance between two atoms, for which a bond
/// population will be generated, when performing a population analysis.
/// # Default
/// 3.0 Å
/// # Example
/// `POPN_BOND_CUTOFF : 2.54 ang`
pub struct PopnBondCutoff {
    cutoff: f64,
    #[keyword_display(is_option = true)]
    unit: Option<LengthUnit>,
}
