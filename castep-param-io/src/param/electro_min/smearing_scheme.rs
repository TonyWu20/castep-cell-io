use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

/// This keyword determines the Fermi-surface smearing scheme to be used if the system is being treated as a metal.
/// Available options are:
/// - Gaussian
/// - GaussianSplines
/// - FermiDirac
/// - HermitePolynomials
/// - ColdSmearing
/// # Note
/// This parameter is used only if FIX_OCCUPANCY : FALSE.
/// # Default
/// Gaussian
/// # Example
/// `SMEARING_SCHEME : ColdSmearing`
#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Default,
    KeywordDisplay,
)]
#[keyword_display(field = "SMEARING_SCHEME")]
pub enum SmearingScheme {
    #[default]
    Gaussian,
    GaussianSplines,
    FermiDirac,
    HermitePolynomials,
    ColdSmearing,
}
