use castep_param_derive::KeywordDisplayStruct;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::PressureUnit;

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, KeywordDisplayStruct, Builder,
)]
#[builder(setter(into, strip_option), default)]
#[keyword_display(
    field = "GEOM_STRESS_TOL",
    display_format = "{:20.15} {}",
    default_value = 0.1,
    from=f64,
)]
/// This keyword determines the tolerance for accepting convergence of the
/// maximum stress component during unit cell optimization.
/// # Default
/// 0.1 GPa
/// # Example
/// `GEOM_STRESS_TOL : 0.2 GPa`
pub struct GeomStressTol {
    tol: f64,
    #[keyword_display(is_option = true)]
    unit: Option<PressureUnit>,
}
