use castep_param_derive::KeywordDisplayStruct;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::LengthUnit;

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, KeywordDisplayStruct, Builder,
)]
#[builder(setter(into, strip_option), default)]
#[keyword_display(field = "GEOM_DISP_TOL", display_format = "{:20.15} {}", from=f64, default_value=0.001)]
/// This keyword determines the tolerance for accepting convergence of the ionic
/// displacement during a geometry optimization.
/// # Default
/// 0.001 Å
/// # Example
/// `GEOM_DISP_TOL : 0.002 ang`
pub struct GeomDispTol {
    pub tol: f64,
    #[keyword_display(is_option = true)]
    pub unit: Option<LengthUnit>,
}
