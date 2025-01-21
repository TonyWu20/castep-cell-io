use castep_param_derive::KeywordDisplayStruct;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::ForceUnit;

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, KeywordDisplayStruct, Builder,
)]
#[builder(setter(into, strip_option), default)]
#[keyword_display(field = "GEOM_FORCE_TOL", display_format = "{:20.15} {}", from=f64, default_value=0.05)]
/// This keyword controls the tolerance for accepting convergence of the ionic
/// force during a geometry optimization.
/// # Default
/// 0.05 eV Å-1
/// # Example
/// `GEOM_FORCE_TOL : 0.07 ev/ang`
pub struct GeomForceTol {
    pub tol: f64,
    #[keyword_display(is_option = true)]
    pub unit: Option<ForceUnit>,
}
