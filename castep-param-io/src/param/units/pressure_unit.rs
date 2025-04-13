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
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromPest,
    BuildFromPairs,
)]
/// This keyword specifies the units in which pressure will be reported.
/// # Example
/// `PRESSURE_UNIT : atm`
#[pest_ast(rule(Rule::pressure_unit))]
#[pest_rule(rule=Rule::pressure_unit,keyword="PRESSURE_UNIT")]
pub enum PressureUnit {
    #[pest_ast(inner(rule(Rule::pressure_units), with(from_span)))]
    HartreePerBohr3,
    ElectronVoltsPerAng3,
    Pascal,
    Megapascal,
    Gigapascal,
    Atmosphere,
    Bar,
    Megabar,
}

fn from_span(span: Span<'_>) -> PressureUnit {
    let input = span.as_str();
    match input.to_lowercase().as_str() {
        "hartree/bohr**3" => Some(PressureUnit::HartreePerBohr3),
        "ev/ang**3" => Some(PressureUnit::ElectronVoltsPerAng3),
        "pa" => Some(PressureUnit::Pascal),
        "mpa" => Some(PressureUnit::Megapascal),
        "gpa" => Some(PressureUnit::Gigapascal),
        _ => None,
    }
    .expect("Always correct")
}
impl Display for PressureUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PressureUnit::HartreePerBohr3 => f.write_str("hartree/bohr**3"),
            PressureUnit::ElectronVoltsPerAng3 => f.write_str("ev/ang**3"),
            PressureUnit::Pascal => f.write_str("pa"),
            PressureUnit::Megapascal => f.write_str("mpa"),
            PressureUnit::Gigapascal => f.write_str("gpa"),
            PressureUnit::Atmosphere => f.write_str("atm"),
            PressureUnit::Bar => f.write_str("bar"),
            PressureUnit::Megabar => f.write_str("mbar"),
        }
    }
}

impl KeywordDisplay for PressureUnit {
    fn field(&self) -> String {
        "PRESSURE_UNIT".to_string()
    }
}
