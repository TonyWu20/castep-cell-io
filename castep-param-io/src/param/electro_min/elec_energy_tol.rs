use castep_param_derive::KeywordDisplayStruct;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::EnergyUnit;

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, KeywordDisplayStruct, Builder,
)]
#[keyword_display(field = "ELEC_ENERGY_TOL", display_format = "{:20.15e} {}", from=f64, default_value=1e-5 )]
#[builder(setter(into), default)]
/// This keyword controls the tolerance for accepting convergence of the total
/// energy in an electronic minimization.
/// # Note
/// The difference between maximum and minimum energies over ELEC_CONVERGENCE_WIN
/// iterations must be less than this value.
/// # Default
/// 1x10-5 eV per atom
pub struct ElecEnergyTol {
    pub tol: f64,
    #[keyword_display(is_option = true)]
    pub unit: Option<EnergyUnit>,
}
