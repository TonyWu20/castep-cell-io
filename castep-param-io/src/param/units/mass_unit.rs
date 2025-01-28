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
/// This keyword specifies the units in which masses will be reported.
/// # Example
/// `MASS_UNIT : kg`
#[pest_ast(rule(Rule::mass_unit))]
#[pest_rule(rule=Rule::mass_unit,keyword="MASS_UNIT")]
pub enum MassUnit {
    #[pest_ast(inner(rule(Rule::mass_units), with(from_span)))]
    ElectronMass,
    #[default]
    AtomicMassUnit,
    Kilogram,
    Gram,
}

fn from_span(span: Span<'_>) -> MassUnit {
    let input = span.as_str();
    match input.to_lowercase().as_str() {
        "me" => Some(MassUnit::ElectronMass),
        "amu" => Some(MassUnit::AtomicMassUnit),
        "kg" => Some(MassUnit::Kilogram),
        "g" => Some(MassUnit::Gram),
        _ => None,
    }
    .expect("Always correct")
}

impl Display for MassUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MassUnit::ElectronMass => f.write_str("me"),
            MassUnit::AtomicMassUnit => f.write_str("amu"),
            MassUnit::Kilogram => f.write_str("kg"),
            MassUnit::Gram => f.write_str("g"),
        }
    }
}

impl KeywordDisplay for MassUnit {
    fn field(&self) -> String {
        "MASS_UNIT".to_string()
    }
}
