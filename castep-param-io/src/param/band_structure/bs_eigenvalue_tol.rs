use castep_param_derive::KeywordDisplayStruct;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::EnergyUnit;

/// This keyword controls the tolerance for accepting convergence of a
/// single eigenvalue or band during a band structure calculation.
/// # Note
/// The difference between maximum and minimum eigenvalue for every band
/// over ELEC_CONVERGENCE_WIN iterations must be less than this value.
/// # Default
/// 1x10-6 eV
/// # Example
/// `BS_EIGENVALUE_TOL = 1.0e-5 Ha`
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Builder, KeywordDisplayStruct,
)]
#[keyword_display(
    field = "BS_EIGENVALUE_TOL",
    from = f64,
    default_value = 1e-6,
    display_format = "{:20.15e} {}",
)]
pub struct BSEigenvalueTol {
    tol: f64,
    #[keyword_display(is_option = true)]
    unit: Option<EnergyUnit>,
}
