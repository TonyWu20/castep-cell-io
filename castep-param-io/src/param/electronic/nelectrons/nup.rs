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
#[keyword_display(field="NUP", from=f64, value=f64)]
#[pest_ast(rule(Rule::nup))]
#[pest_rule(rule=Rule::nup,keyword="NUP")]
pub struct NUp(#[pest_ast(inner(rule(Rule::real), with(Real::from_span), with(f64::from)))] f64);
