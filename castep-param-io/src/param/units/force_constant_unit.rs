use crate::parser::Rule;
use castep_param_derive::BuildFromPairs;
use from_pest::FromPest;
use pest::{Parser, Span};
use pest_ast::FromPest;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

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
#[pest_ast(rule(Rule::force_constant_unit))]
#[pest_rule(rule=Rule::force_constant_unit, keyword="FORCE_CONSTANT_UNIT")]
/// This keyword specifies the units in which force constants will be reported.
/// # Example
/// `FORCE_CONSTANT_UNIT : n/m`
pub enum ForceConstantUnit {
    #[pest_ast(inner(rule(Rule::force_constant_units), with(from_span)))]
    HartreePerBohr2,
    #[default]
    ElectronVoltsPerAng2,
    NewtonPerMeter,
    DynesPerCentimeter,
}

fn from_span(span: Span<'_>) -> ForceConstantUnit {
    let input = span.as_str();
    match input.to_lowercase().as_str() {
        "hartree/bohr**2" => Some(ForceConstantUnit::HartreePerBohr2),
        "ev/ang**2" => Some(ForceConstantUnit::ElectronVoltsPerAng2),
        "n/m" => Some(ForceConstantUnit::NewtonPerMeter),
        "dyne/cm" => Some(ForceConstantUnit::DynesPerCentimeter),
        _ => None,
    }
    .expect("Always correct")
}

impl Display for ForceConstantUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForceConstantUnit::HartreePerBohr2 => f.write_str("hartree/bohr**2"),
            ForceConstantUnit::ElectronVoltsPerAng2 => f.write_str("ev/ang**2"),
            ForceConstantUnit::NewtonPerMeter => f.write_str("n/m"),
            ForceConstantUnit::DynesPerCentimeter => f.write_str("dyne/cm"),
        }
    }
}

impl KeywordDisplay for ForceConstantUnit {
    fn field(&self) -> String {
        "FORCE_CONSTANT_UNIT".to_string()
    }
}

#[cfg(test)]
mod test {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::parser::{ParamParser, Rule};

    use super::ForceConstantUnit;

    #[test]
    fn force_constant_unit_parse() {
        let input = "force_constant_unit : hartree/bohr**2";
        let mut parse = ParamParser::parse(Rule::force_constant_unit, input).unwrap();
        let unit = ForceConstantUnit::from_pest(&mut parse).unwrap();
        dbg!(unit);
    }
}
