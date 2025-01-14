use castep_param_derive::KeywordDisplayStruct;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::PressureUnit;

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, KeywordDisplayStruct, Builder,
)]
#[builder(setter(into, strip_option), default)]
#[keyword_display(field = "GEOM_MODULUS_EST", display_format = "{:20.15} {}", from=f64,default_value=500.0)]
/// This keyword provides an estimate of the bulk modulus of the system.
/// It is used to initialize the Hessian for BFGS geometry optimization with
/// cell relaxation.
/// # Default
/// 500.0 GPa
/// # Example
/// `GEOM_MODULUS_EST : 125.4 GPa`
pub struct GeomModulusEst {
    tol: f64,
    #[keyword_display(is_option = true)]
    unit: Option<PressureUnit>,
}
