use crate::parser::{data_type::Real, Rule};
use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

/// This keyword determines the size of the standard grid, relative to the diameter of the cutoff sphere.
/// # Default
/// `1.75`
/// # Example
/// `GRID_SCALE : 2.0`
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
    KeywordDisplay,
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(field="GRID_SCALE",
    from=f64,
    value=f64,
    display_format="{:20.15}",
    default_value=1.75
)]
#[pest_ast(rule(Rule::grid_scale))]
#[pest_rule(rule=Rule::grid_scale,keyword="GRID_SCALE")]
pub struct GridScale(
    #[pest_ast(inner(rule(Rule::real), with(Real::from_span), with(f64::from)))] f64,
);
