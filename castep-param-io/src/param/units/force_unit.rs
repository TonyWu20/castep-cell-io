use castep_param_derive::BuildFromPairs;
use from_pest::FromPest;
use pest::{Parser, Span};
use std::fmt::Display;

use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::{param::KeywordDisplay, parser::Rule};

#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    Default,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromPest,
    BuildFromPairs,
)]
/// This keyword specifies the units in which force will be reported.
/// # Example
/// `FORCE_UNIT : n`
#[pest_ast(rule(Rule::force_unit))]
#[pest_rule(rule=Rule::force_unit, keyword="FORCE_UNIT")]
pub enum ForceUnit {
    #[pest_ast(inner(rule(Rule::force_units), with(from_span)))]
    HartreePerBohr,
    #[default]
    ElectronVoltsPerAng,
    Newton,
}

fn from_span(value: Span<'_>) -> ForceUnit {
    let input = value.as_str();
    match input.to_lowercase().as_str() {
        "hartree/bohr" => Some(ForceUnit::HartreePerBohr),
        "ev/ang" => Some(ForceUnit::ElectronVoltsPerAng),
        "n" => Some(ForceUnit::Newton),
        _ => None,
    }
    .expect("AlwaysCorrect")
}

impl Display for ForceUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForceUnit::HartreePerBohr => f.write_str("hartree/bohr"),
            ForceUnit::ElectronVoltsPerAng => f.write_str("ev/ang"),
            ForceUnit::Newton => f.write_str("n"),
        }
    }
}

impl KeywordDisplay for ForceUnit {
    fn field(&self) -> String {
        "FORCE_UNIT".to_string()
    }
}
