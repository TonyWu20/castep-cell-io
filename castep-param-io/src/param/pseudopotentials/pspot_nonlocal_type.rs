use std::fmt::Display;

use crate::parser::Rule;
use castep_param_derive::BuildFromPairs;
use from_pest::FromPest;
use pest::{Parser, Span};
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Default,
    FromPest,
    BuildFromPairs,
)]
/// This keyword controls the representation of the nonlocal part of the pseudopotential.
/// Available options are:
/// - RECIPROCAL - reciprocal space nonlocal pseudopotentials.
/// - REAL - real space nonlocal pseudopotentials.
/// # Default
/// The default is the value of `RECIPROCAL`.
/// # Example
/// `PSPOT_NONLOCAL_TYPE : REAL`
#[pest_ast(rule(Rule::pspot_nonlocal_type))]
#[pest_rule(rule=Rule::pspot_nonlocal_type,keyword="PSPOT_NONLOCAL_TYPE")]
pub enum PSPotNonlocalType {
    #[pest_ast(inner(rule(Rule::pspot_nonlocal_types), with(from_span)))]
    #[default]
    Reciprocal,
    Real,
}

fn from_span(span: Span<'_>) -> PSPotNonlocalType {
    let input = span.as_str();
    match input.to_lowercase().as_str() {
        "reciprocal" => Some(PSPotNonlocalType::Reciprocal),
        "real" => Some(PSPotNonlocalType::Real),
        _ => None,
    }
    .expect("Always correct")
}

impl Display for PSPotNonlocalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PSPotNonlocalType::Reciprocal => f.write_str("reciprocal"),
            PSPotNonlocalType::Real => f.write_str("real"),
        }
    }
}

impl KeywordDisplay for PSPotNonlocalType {
    fn field(&self) -> String {
        "PSPOT_NONLOCAL_TYPE".to_string()
    }
}
