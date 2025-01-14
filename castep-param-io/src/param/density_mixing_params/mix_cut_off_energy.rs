use castep_param_derive::KeywordDisplayStruct;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::EnergyUnit;

/// This keyword determines the cutoff energy for the densities used in the
/// density mixing scheme.
/// The value specified determines the extent of the grid used for mixing old
/// and new densities. Density components with wave vectors higher than that specified are not mixed.
/// # Default
/// The default value is the CUT_OFF_ENERGY.
/// # Example
/// `MIX_CUT_OFF_ENERGY : 250.0 eV`
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
    Default,
    KeywordDisplayStruct,
    Builder,
)]
#[keyword_display(field = "MIX_CUT_OFF_ENERGY", display_format = "{:20.15} {}", from=f64)]
pub struct MixCutOffEnergy {
    pub cutoff_energy: f64,
    #[keyword_display(is_option = true)]
    pub unit: Option<EnergyUnit>,
}
#[cfg(test)]
mod test {
    use crate::param::KeywordDisplay;

    use super::MixCutOffEnergy;

    #[test]
    fn mix_cut_off_energy() {
        let mix_cutoff = MixCutOffEnergy {
            cutoff_energy: 250.0,
            unit: None,
        };
        println!("{}", mix_cutoff.output());
    }
}
