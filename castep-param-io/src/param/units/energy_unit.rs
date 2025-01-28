use from_pest::FromPest;
use pest::Parser;
use std::fmt::Display;

use castep_param_derive::BuildFromPairs;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::{
    param::KeywordDisplay,
    parser::{span_into_str, Rule},
};
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
/// This keyword specifies the units in which energies will be reported.
/// # Example
/// `ENERGY_UNIT : kcal/mol`
#[pest_ast(rule(Rule::energy_unit))]
#[pest_rule(rule=Rule::energy_unit, keyword="ENERGY_UNIT")]
pub enum EnergyUnit {
    #[pest_ast(inner(
        rule(Rule::energy_units),
        with(span_into_str),
        with(EnergyUnit::from_str),
        with(Option::unwrap)
    ))]
    Hartree,
    Millihartree,
    #[default]
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
    Wavenumber,
    Kelvin,
}
impl Display for EnergyUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnergyUnit::Hartree => f.write_str("ha"),
            EnergyUnit::Millihartree => f.write_str("mha"),
            EnergyUnit::ElectronVolt => f.write_str("eV"),
            EnergyUnit::MillielectronVolt => f.write_str("meV"),
            EnergyUnit::Rydberg => f.write_str("ry"),
            EnergyUnit::Millirydberg => f.write_str("mry"),
            EnergyUnit::KilojoulesPerMole => f.write_str("kj/mol"),
            EnergyUnit::KilocaloriesPerMole => f.write_str("kcal/mol"),
            EnergyUnit::Joules => f.write_str("j"),
            EnergyUnit::Erg => f.write_str("erg"),
            EnergyUnit::Hertz => f.write_str("hz"),
            EnergyUnit::Megahertz => f.write_str("mhz"),
            EnergyUnit::Gigahertz => f.write_str("ghz"),
            EnergyUnit::Terahertz => f.write_str("thz"),
            EnergyUnit::Wavenumber => f.write_str("cm-1"),
            EnergyUnit::Kelvin => f.write_str("k"),
        }
    }
}

impl KeywordDisplay for EnergyUnit {
    fn field(&self) -> String {
        "ENERGY_UNIT".to_string()
    }
}

impl EnergyUnit {
    fn from_str(input: &str) -> Option<Self> {
        match input {
            "ha" => Some(EnergyUnit::Hartree),
            "mha" => Some(EnergyUnit::Millihartree),
            "eV" => Some(EnergyUnit::ElectronVolt),
            "meV" => Some(EnergyUnit::MillielectronVolt),
            "ry" => Some(EnergyUnit::Rydberg),
            "mry" => Some(EnergyUnit::Millirydberg),
            "kj/mol" => Some(EnergyUnit::KilojoulesPerMole),
            "kcal/mol" => Some(EnergyUnit::KilocaloriesPerMole),
            "j" => Some(EnergyUnit::Joules),
            "erg" => Some(EnergyUnit::Erg),
            "hz" => Some(EnergyUnit::Hertz),
            "mhz" => Some(EnergyUnit::Megahertz),
            "ghz" => Some(EnergyUnit::Gigahertz),
            "thz" => Some(EnergyUnit::Terahertz),
            "cm-1" => Some(EnergyUnit::Wavenumber),
            "k" => Some(EnergyUnit::Kelvin),
            _ => None,
        }
    }
}
