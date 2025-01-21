use crate::param::KeywordDisplay;
use castep_param_derive::ParamDisplay;
use derive_builder::Builder;
use pdos_calculate_weights::PDOSCalculateWeights;
use popn_bond_cutoff::PopnBondCutoff;
use popn_calculate::PopnCalculate;
use popn_write::PopnWrite;
use serde::{Deserialize, Serialize};

mod pdos_calculate_weights;
mod popn_bond_cutoff;
mod popn_calculate;
mod popn_write;

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    PartialOrd,
    Copy,
    ParamDisplay,
    Builder,
    Default,
)]
#[builder(setter(into, strip_option), default)]
pub struct PopulationAnalysis {
    pub pdos_calculate_weights: Option<PDOSCalculateWeights>,
    pub popn_bond_cutoff: Option<PopnBondCutoff>,
    pub popn_calculate: Option<PopnCalculate>,
    pub popn_write: Option<PopnWrite>,
}

#[cfg(test)]
mod test {
    use super::{
        popn_bond_cutoff::PopnBondCutoff, popn_write::PopnWrite, PopulationAnalysisBuilder,
    };

    #[test]
    fn popn_analysis() {
        let p = PopulationAnalysisBuilder::default()
            .pdos_calculate_weights(true)
            .popn_bond_cutoff(PopnBondCutoff::default())
            .popn_calculate(true)
            .popn_write(PopnWrite::Summary)
            .build()
            .unwrap();
        println!("{p}")
    }
}
