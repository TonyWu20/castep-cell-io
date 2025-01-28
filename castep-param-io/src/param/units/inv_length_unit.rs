use castep_param_derive::BuildFromPairs;
use from_pest::FromPest;
use pest::{Parser, Span};
use std::fmt::Display;

use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;
use crate::parser::Rule;

#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    FromPest,
    BuildFromPairs,
)]
/// This keyword specifies the units in which inverse length will be reported.
/// # Example
/// `INV_LENGTH_UNIT : 1/nm`
#[pest_ast(rule(Rule::inv_length_unit))]
#[pest_rule(rule=Rule::inv_length_unit,keyword="INV_LENGTH_UNIT")]
pub enum InvLengthUnit {
    #[pest_ast(inner(rule(Rule::inv_length_units), with(from_span)))]
    Bohr,
    Meter,
    Nanometer,
    #[default]
    Ang,
}

fn from_span(span: Span<'_>) -> InvLengthUnit {
    let input = span.as_str();
    match input.to_lowercase().as_str() {
        "1/" => Some(InvLengthUnit::Bohr),
        "1/m" => Some(InvLengthUnit::Meter),
        "1/nm" => Some(InvLengthUnit::Nanometer),
        "1/ang" => Some(InvLengthUnit::Ang),
        _ => None,
    }
    .expect("Always correct")
}

impl Display for InvLengthUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvLengthUnit::Bohr => f.write_str("1/"),
            InvLengthUnit::Meter => f.write_str("1/m"),
            InvLengthUnit::Nanometer => f.write_str("1/nm"),
            InvLengthUnit::Ang => f.write_str("1/ang"),
        }
    }
}

impl KeywordDisplay for InvLengthUnit {
    fn field(&self) -> String {
        "INV_LENGTH_UNIT".to_string()
    }
}
