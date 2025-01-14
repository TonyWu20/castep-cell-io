use castep_param_derive::KeywordDisplayStruct;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::EnergyUnit;
/// This keyword determines the spacing of cutoff energies used to estimate
/// the `BasisDeDloge` in automatic calculation of the finite basis set correction.
/// Points are chosen such that the `CutoffEnergy` corresponds to the highest energy
/// in the set of `FiniteBasisNPoints` cutoff energies.
/// # Note
/// This value is only used if FINITE_BASIS_CORR : 2.
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Builder, KeywordDisplayStruct,
)]
#[builder(setter(into), default)]
#[keyword_display(field = "FINITE_BASIS_SPACING", from=f64, default_value=5.0, display_format="{:20.15} {}")]
pub struct FiniteBasisSpacing {
    spacing: f64,
    #[keyword_display(is_option = true)]
    unit: Option<EnergyUnit>,
}
