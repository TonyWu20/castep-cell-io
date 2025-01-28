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
/// This keyword specifies the units in which frequency will be reported.
/// # Example
/// `FREQUENCY_UNIT : hz`
#[pest_ast(rule(Rule::frequency_unit))]
#[pest_rule(rule=Rule::frequency_unit,keyword="FREQUENCY_UNIT")]
pub enum FrequencyUnit {
    #[pest_ast(inner(rule(Rule::frequency_units), with(from_span)))]
    Hartree,
    Millihartree,
    ElectronVolt,
    MillielectronVolt,
    Rydberg,
    Millirydberg,
    KilojoulesPerMole,
    KilocaloriesPerMole,
    Joules,
    Erg,
    Hertz,
    Megahertz,
    Gigahertz,
    Terahertz,
    #[default]
    Wavenumber,
    Kelvin,
}

fn from_span(span: Span<'_>) -> FrequencyUnit {
    let input = span.as_str();
    match input.to_lowercase().as_str() {
        "ha" => Some(FrequencyUnit::Hartree),
        "mha" => Some(FrequencyUnit::Millihartree),
        "ev" => Some(FrequencyUnit::ElectronVolt),
        "mev" => Some(FrequencyUnit::MillielectronVolt),
        "ry" => Some(FrequencyUnit::Rydberg),
        "mry" => Some(FrequencyUnit::Millirydberg),
        "kj/mol" => Some(FrequencyUnit::KilojoulesPerMole),
        "kcal/mol" => Some(FrequencyUnit::KilocaloriesPerMole),
        "j" => Some(FrequencyUnit::Joules),
        "erg" => Some(FrequencyUnit::Erg),
        "hz" => Some(FrequencyUnit::Hertz),
        "mhz" => Some(FrequencyUnit::Megahertz),
        "ghz" => Some(FrequencyUnit::Gigahertz),
        "thz" => Some(FrequencyUnit::Terahertz),
        "cm-1" => Some(FrequencyUnit::Wavenumber),
        "k" => Some(FrequencyUnit::Kelvin),
        _ => None,
    }
    .expect("Always correct")
}

impl Display for FrequencyUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrequencyUnit::Hartree => f.write_str("ha"),
            FrequencyUnit::Millihartree => f.write_str("mha"),
            FrequencyUnit::ElectronVolt => f.write_str("ev"),
            FrequencyUnit::MillielectronVolt => f.write_str("mev"),
            FrequencyUnit::Rydberg => f.write_str("ry"),
            FrequencyUnit::Millirydberg => f.write_str("mry"),
            FrequencyUnit::KilojoulesPerMole => f.write_str("kj/mol"),
            FrequencyUnit::KilocaloriesPerMole => f.write_str("kcal/mol"),
            FrequencyUnit::Joules => f.write_str("j"),
            FrequencyUnit::Erg => f.write_str("erg"),
            FrequencyUnit::Hertz => f.write_str("hz"),
            FrequencyUnit::Megahertz => f.write_str("mhz"),
            FrequencyUnit::Gigahertz => f.write_str("ghz"),
            FrequencyUnit::Terahertz => f.write_str("thz"),
            FrequencyUnit::Wavenumber => f.write_str("cm-1"),
            FrequencyUnit::Kelvin => f.write_str("k"),
        }
    }
}

impl KeywordDisplay for FrequencyUnit {
    fn field(&self) -> String {
        "FREQUENCY_UNIT".to_string()
    }
}
