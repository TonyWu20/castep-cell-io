use castep_param_derive::KeywordDisplayStruct;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::EnergyUnit;

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, KeywordDisplayStruct, Builder,
)]
#[builder(setter(into, strip_option), default)]
#[keyword_display(field = "GEOM_ENERGY_TOL", display_format = "{:20.15} {}", from=f64, default_value=2e-5)]
/// This keyword controls the tolerance for accepting convergence of the free energy
/// per atom during a geometry optimization.
/// # Note
/// The difference between maximum and minimum values of the free energy over
/// GEOM_CONVERGENCE_WIN iterations must be less than this value.
/// # Default
/// 2×10-5 eV per atom
/// # Example
/// `GEOM_ENERGY_TOL : 0.00005 eV`
pub struct GeomEnergyTol {
    pub tol: f64,
    #[keyword_display(is_option = true)]
    pub unit: Option<EnergyUnit>,
}
