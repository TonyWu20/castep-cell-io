#![allow(dead_code)]
use std::fmt::Display;

use band_structure::BandStructure;
use castep_param_derive::ParamDisplay;

mod band_structure;
mod basis_set;
mod density_mixing_params;
mod electro_min;
mod electronic;
mod general;
mod geom_opt;
mod population_analysis;
mod pseudopotentials;
#[cfg(feature = "template")]
mod template;
mod units;
mod xc_correlation;

pub use basis_set::*;
pub use density_mixing_params::*;
pub use electro_min::*;
pub use electronic::*;
pub use general::*;
pub use geom_opt::*;
pub use population_analysis::*;
pub use pseudopotentials::*;
pub use units::*;
pub use xc_correlation::*;

#[derive(
    Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, ParamDisplay, Builder, Default,
)]
#[param_display(use_display = true)]
#[builder(setter(strip_option), default)]
pub struct CastepParam {
    #[param_display(use_ref = true)]
    pub general_keywords: Option<General>,
    pub units: Option<Units>,
    pub pseudopotentials: Option<Pseudopotentials>,
    pub band_structure: Option<BandStructure>,
    pub basis_set: Option<BasisSet>,
    pub electronic: Option<ElectronicParam>,
    #[param_display(use_ref = true)]
    pub electro_min: Option<ElectroMinimization>,
    pub density_mixing: Option<DensityMixingParams>,
    pub population_analysis: Option<PopulationAnalysis>,
    pub geom_opt: Option<GeomOpt>,
    #[param_display(use_ref = true)]
    pub xc_correlation: Option<XcParam>,
}

pub trait KeywordDisplay: Display {
    fn field(&self) -> String;
    fn output(&self) -> String {
        format!("{} : {}", self.field(), self)
    }
}

pub trait ListFields {
    fn list_fields() -> Vec<String>;
}
