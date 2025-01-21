use castep_param_derive::KeywordDisplayStruct;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::FrequencyUnit;

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, KeywordDisplayStruct, Builder,
)]
#[builder(setter(into, strip_option), default)]
#[keyword_display(field = "GEOM_FREQUENCY_EST", display_format = "{:20.15} {}", from=f64, default_value=50.0)]
/// This keyword provides an estimate of the average phonon frequency at the gamma
/// point. It is used to initialize the Hessian for BFGS geometry optimization with
/// ionic relaxation. One expects to achieve faster convergence of the geometry
/// optimization if the value is realistic. The idea is that this single number
/// represents the whole of the Hessian in a compact form.
/// The value of the estimated frequency is provided in the CASTEP output after
/// the geometry optimization run, and this value can be used as an input parameter
/// for a new calculation on a similar system. An example would be a study of
/// different configurations of a self-interstitial defect in silicon.
/// # Note
/// The value of the estimated frequency as returned by CASTEP should not be
/// interpreted as an actual frequency of any real vibrational mode.
/// # Default
/// 50 THz
/// # Example
/// `GEOM_FREQUENCY_EST : 17.54 THz`
pub struct GeomFrequencyEst {
    pub est: f64,
    #[keyword_display(is_option = true)]
    pub unit: Option<FrequencyUnit>,
}
