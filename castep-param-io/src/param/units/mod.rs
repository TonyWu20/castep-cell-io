use super::KeywordDisplay;
use castep_param_derive::{ParamDisplay, StructBuildFromPairs};
use derive_builder::Builder;

pub use energy_unit::EnergyUnit;
pub use force_constant_unit::ForceConstantUnit;
pub use force_unit::ForceUnit;
pub use frequency_unit::FrequencyUnit;
pub use inv_length_unit::InvLengthUnit;
pub use length_unit::LengthUnit;
pub use mass_unit::MassUnit;
pub use pressure_unit::PressureUnit;
pub use serde::{Deserialize, Serialize};
pub use time_unit::TimeUnit;
pub use velocity_unit::VelocityUnit;
pub use volume_unit::VolumeUnit;

mod energy_unit;
mod force_constant_unit;
mod force_unit;
mod frequency_unit;
mod inv_length_unit;
mod length_unit;
mod mass_unit;
mod pressure_unit;
mod time_unit;
mod velocity_unit;
mod volume_unit;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Default,
    Hash,
    Builder,
    ParamDisplay,
    StructBuildFromPairs,
)]
#[builder(setter(into, strip_option), default)]
pub struct Units {
    pub energy_unit: Option<EnergyUnit>,
    pub force_unit: Option<ForceUnit>,
    pub force_constant_unit: Option<ForceConstantUnit>,
    pub frequency_unit: Option<FrequencyUnit>,
    pub inv_length_unit: Option<InvLengthUnit>,
    pub length_unit: Option<LengthUnit>,
    pub mass_unit: Option<MassUnit>,
    pub pressure_unit: Option<PressureUnit>,
    pub time_unit: Option<TimeUnit>,
    pub velocity_unit: Option<VelocityUnit>,
    pub volume_unit: Option<VolumeUnit>,
}

#[cfg(test)]
mod test {
    use crate::param::units::{LengthUnit, UnitsBuilder};

    #[test]
    fn units() {
        let unit = UnitsBuilder::default()
            .length_unit(LengthUnit::Nanometer)
            .build()
            .unwrap();
        println!("output: {}", unit);
    }
}
