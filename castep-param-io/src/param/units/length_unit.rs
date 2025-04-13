use castep_param_derive::BuildFromPairs;
use from_pest::FromPest;
use pest::{Parser, Span};
use pest_ast::FromPest;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{param::KeywordDisplay, parser::Rule};

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
/// This keyword specifies the units in which lengths will be reported.
/// # Example
/// `LENGTH_UNIT : bohr`
#[pest_ast(rule(Rule::length_unit))]
#[pest_rule(rule=Rule::length_unit,keyword="LENGTH_UNIT")]
pub enum LengthUnit {
    #[pest_ast(inner(rule(Rule::length_units), with(from_span)))]
    Bohr,
    BohrA0,
    Meter,
    Centimeter,
    Nanometer,
    #[default]
    Ang,
}

fn from_span(span: Span<'_>) -> LengthUnit {
    let input = span.as_str();
    match input.to_lowercase().as_str() {
        "bohr" => Some(LengthUnit::Bohr),
        "a0" => Some(LengthUnit::BohrA0),
        "m" => Some(LengthUnit::Meter),
        "cm" => Some(LengthUnit::Centimeter),
        "nm" => Some(LengthUnit::Nanometer),
        "ang" => Some(LengthUnit::Ang),
        _ => None,
    }
    .expect("Always correct")
}

impl Display for LengthUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LengthUnit::Bohr => f.write_str("bohr"),
            LengthUnit::BohrA0 => f.write_str("a0"),
            LengthUnit::Meter => f.write_str("m"),
            LengthUnit::Centimeter => f.write_str("cm"),
            LengthUnit::Nanometer => f.write_str("nm"),
            LengthUnit::Ang => f.write_str("ang"),
        }
    }
}

impl KeywordDisplay for LengthUnit {
    fn field(&self) -> String {
        "LENGTH_UNIT".to_string()
    }
}
