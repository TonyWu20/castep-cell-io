use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    KeywordDisplay,
    Default,
)]
#[keyword_display(field = "GEOM_METHOD")]
#[allow(clippy::upper_case_acronyms)]
/// This keyword determines the method used for geometry optimization.
/// Available options are:
/// - BFGS - BFGS minimization.
/// - LBFGS - low-memory BFGS minimization.
/// - Delocalized (or Delocalised) - BFGS minimization using delocalized internal coordinates instead of Cartesian coordinates.
/// - DampedMD - Damped molecular dynamics.
/// - TPSD - Two-point steepest descent.
/// - # Default
/// - BFGS
/// - # Example
/// - `GEOM_METHOD : DampedMD`
pub enum GeomMethod {
    #[default]
    BFGS,
    LBFGS,
    Delocalized,
    DampedMD,
    TPSD,
}
