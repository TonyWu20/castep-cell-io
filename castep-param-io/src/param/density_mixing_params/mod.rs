use crate::param::KeywordDisplay;
use castep_param_derive::ParamDisplay;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

mod mix_charge_amp;
mod mix_charge_gmax;
mod mix_cut_off_energy;
mod mix_history_length;
mod mix_metric_q;
mod mix_spin_amp;
mod mix_spin_gmax;
mod mixing_scheme;

pub use mix_charge_amp::MixChargeAmp;
pub use mix_charge_gmax::*;
pub use mix_cut_off_energy::*;
pub use mix_history_length::MixHistoryLength;
pub use mix_metric_q::*;
pub use mix_spin_amp::MixSpinAmp;
pub use mix_spin_gmax::*;
pub use mixing_scheme::MixingScheme;

#[derive(
    Debug,
    Clone,
    Copy,
    Builder,
    Default,
    ParamDisplay,
    Serialize,
    Deserialize,
    PartialEq,
    PartialOrd,
)]
#[builder(setter(into, strip_option), default)]
pub struct DensityMixingParams {
    pub mixing_scheme: Option<MixingScheme>,
    pub mix_charge_amp: Option<MixChargeAmp>,
    pub mix_spin_amp: Option<MixSpinAmp>,
    pub mix_charge_gmax: Option<MixChargeGmax>,
    pub mix_spin_gmax: Option<MixSpinGmax>,
    pub mix_cut_off_energy: Option<MixCutOffEnergy>,
    pub mix_metric_q: Option<MixMetricQ>,
    pub mix_history_length: Option<MixHistoryLength>,
}

#[cfg(test)]
mod test {
    use super::{DensityMixingParamsBuilder, MixChargeGmax, MixSpinGmax, MixingScheme};

    #[test]
    fn dm_param() {
        let p = DensityMixingParamsBuilder::default()
            .mix_spin_amp(2.0)
            .mix_spin_gmax(MixSpinGmax::default())
            .mix_charge_amp(0.5)
            .mix_charge_gmax(MixChargeGmax {
                gmax: 1.5,
                unit: None,
            })
            .mix_history_length(20)
            .mixing_scheme(MixingScheme::Pulay)
            .build()
            .unwrap();
        println!("{p}");
    }
}
