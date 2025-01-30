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
/// This keyword specifies the units in which velocity will be reported.
/// # Example
/// `VELOCITY_UNIT : bohr/fs`
#[pest_ast(rule(Rule::velocity_unit))]
#[pest_rule(rule=Rule::velocity_unit,keyword="VELOCITY_UNIT")]
pub enum VelocityUnit {
    #[pest_ast(inner(rule(Rule::velocity_units), with(from_span)))]
    AtomicUnitOfVelocity,
    #[default]
    AngPerPs,
    AngPerFs,
    BohrPerPs,
    BohrPerFs,
    MetersPerSecond,
}

fn from_span(span: Span<'_>) -> VelocityUnit {
    let input = span.as_str();
    match input.to_lowercase().as_str() {
        "auv" => Some(VelocityUnit::AtomicUnitOfVelocity),
        "ang/ps" => Some(VelocityUnit::AngPerPs),
        "ang/fs" => Some(VelocityUnit::AngPerFs),
        "bohr/ps" => Some(VelocityUnit::BohrPerPs),
        "bohr/fs" => Some(VelocityUnit::BohrPerFs),
        "m/s" => Some(VelocityUnit::MetersPerSecond),
        _ => None,
    }
    .expect("Always correct")
}

impl Display for VelocityUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VelocityUnit::AtomicUnitOfVelocity => f.write_str("auv"),
            VelocityUnit::AngPerPs => f.write_str("ang/ps"),
            VelocityUnit::AngPerFs => f.write_str("ang/fs"),
            VelocityUnit::BohrPerPs => f.write_str("bohr/ps"),
            VelocityUnit::BohrPerFs => f.write_str("bohr/fs"),
            VelocityUnit::MetersPerSecond => f.write_str("m/s"),
        }
    }
}

impl KeywordDisplay for VelocityUnit {
    fn field(&self) -> String {
        "VELOCITY_UNIT".to_string()
    }
}
