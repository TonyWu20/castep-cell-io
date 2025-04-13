use crate::parser::{data_type::Real, Rule};
use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

mod ndown;
mod nup;

pub use ndown::NDown;
pub use nup::NUp;

#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    PartialOrd,
    KeywordDisplay,
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(from=f64, value=f64, field="NELECTRONS")]
#[pest_ast(rule(Rule::nelectrons))]
#[pest_rule(rule=Rule::nelectrons,keyword="NELECTRONS")]
pub struct NElectrons(
    #[pest_ast(inner(rule(Rule::real), with(Real::from_span), with(f64::from)))] f64,
);
