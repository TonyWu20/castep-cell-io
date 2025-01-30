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
/// This keyword specifies the units in which volume will be reported.
/// # Example
/// ` VOLUME_UNIT : nm**3`
#[pest_ast(rule(Rule::volume_unit))]
#[pest_rule(rule=Rule::volume_unit,keyword="VOLUME_UNIT")]
pub enum VolumeUnit {
    #[pest_ast(inner(rule(Rule::volume_units), with(from_span)))]
    Bohr3,
    Meter3,
    Centimeter3,
    Nanometer3,
    #[default]
    Ang3,
}

fn from_span(span: Span<'_>) -> VolumeUnit {
    let input = span.as_str();
    match input.to_lowercase().as_str() {
        "bohr**3" => Some(VolumeUnit::Bohr3),
        "m**3" => Some(VolumeUnit::Meter3),
        "cm**3" => Some(VolumeUnit::Centimeter3),
        "nm**3" => Some(VolumeUnit::Nanometer3),
        "ang**3" => Some(VolumeUnit::Ang3),
        _ => None,
    }
    .expect("Always correct")
}

impl Display for VolumeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VolumeUnit::Bohr3 => f.write_str("bohr**3"),
            VolumeUnit::Meter3 => f.write_str("m**3"),
            VolumeUnit::Centimeter3 => f.write_str("cm**3"),
            VolumeUnit::Nanometer3 => f.write_str("nm**3"),
            VolumeUnit::Ang3 => f.write_str("ang**3"),
        }
    }
}

impl KeywordDisplay for VolumeUnit {
    fn field(&self) -> String {
        "VOLUME_UNIT".to_string()
    }
}

#[cfg(test)]
mod test {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::{
        param::KeywordDisplay,
        parser::{ConsumeKVPairs, ParamFile, ParamParser, Rule},
    };

    use super::VolumeUnit;

    #[test]
    fn volume_unit_parse() {
        let input = VolumeUnit::Bohr3.output();
        let mut parse = ParamParser::parse(Rule::param_file, &input).unwrap();
        let file = ParamFile::from_pest(&mut parse).unwrap();
        let unit = VolumeUnit::find_from_pairs(file.items());
        dbg!(unit);
    }
}
