use crate::parser::{data_type::Real, Rule};
use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

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
#[keyword_display(field="NDOWN", from=f64, value=f64)]
#[pest_ast(rule(Rule::ndown))]
#[pest_rule(rule=Rule::ndown,keyword="NDOWN")]
pub struct NDown(#[pest_ast(inner(rule(Rule::real), with(Real::from_span), with(f64::from)))] f64);
