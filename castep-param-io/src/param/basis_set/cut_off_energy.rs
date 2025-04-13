use crate::parser::{data_type::Real, Rule};
use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

/// This keyword specifies the cutoff energy for the plane wave basis sets that will be used in the calculation.
/// If the BASIS_PRECISION is defined, the cutoff energy will be equal to the highest of the cutoff energies associated with the chosen level of accuracy, for the pseudopotentials used in the calculation.
/// If neither the BASIS_PRECISION nor the CUT_OFF_ENERGY are defined, the default cutoff energy is that associated with the FINE level of accuracy, for the pseudopotentials in the calculation.
/// # Note
/// It is not possible to specify both the BASIS_PRECISION and the CUT_OFF_ENERGY in a single file.
#[derive(
    Debug,
    Clone,
    Copy,
    Deserialize,
    Serialize,
    PartialEq,
    PartialOrd,
    Default,
    KeywordDisplay,
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(field="CUT_OFF_ENERGY", from=f64, value=f64, display_format="{:20.15}")]
#[pest_ast(rule(Rule::cut_off_energy))]
#[pest_rule(rule=Rule::cut_off_energy,keyword="CUT_OFF_ENERGY")]
pub struct CutoffEnergy(
    #[pest_ast(inner(rule(Rule::real), with(Real::from_span), with(f64::from)))] f64,
);
