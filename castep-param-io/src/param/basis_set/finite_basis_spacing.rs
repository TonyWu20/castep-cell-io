use crate::parser::{data_type::Real, Rule};
use castep_param_derive::{BuildFromPairs, KeywordDisplayStruct};
use derive_builder::Builder;
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::param::EnergyUnit;
/// This keyword determines the spacing of cutoff energies used to estimate
/// the `BasisDeDloge` in automatic calculation of the finite basis set correction.
/// Points are chosen such that the `CutoffEnergy` corresponds to the highest energy
/// in the set of `FiniteBasisNPoints` cutoff energies.
/// # Note
/// This value is only used if FINITE_BASIS_CORR : 2.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
    Builder,
    KeywordDisplayStruct,
    FromPest,
    BuildFromPairs,
)]
#[builder(setter(into), default)]
#[keyword_display(field = "FINITE_BASIS_SPACING", from=f64, default_value=5.0, display_format="{:20.15} {}")]
#[pest_ast(rule(Rule::finite_basis_spacing))]
#[pest_rule(rule=Rule::finite_basis_spacing,keyword="FINITE_BASIS_SPACING")]
pub struct FiniteBasisSpacing {
    #[pest_ast(inner(rule(Rule::real), with(Real::from_span), with(f64::from)))]
    spacing: f64,
    #[keyword_display(is_option = true)]
    #[pest_ast(inner(rule(Rule::finite_basis_spacing_units), with(EnergyUnit::from_span)))]
    unit: Option<EnergyUnit>,
}
