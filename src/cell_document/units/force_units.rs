use std::fmt::Display;

use crate::CellParseError;

use super::ParsableUnit;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ForceUnits {
    HartreePerBohr,
    #[default]
    ElectronVoltsPerAng,
    Newton,
}

impl Display for ForceUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForceUnits::HartreePerBohr => f.write_str("hartree/bohr"),
            ForceUnits::ElectronVoltsPerAng => f.write_str("ev/ang"),
            ForceUnits::Newton => f.write_str("n"),
        }
    }
}

impl ParsableUnit for ForceUnits {
    fn parse_from_str(input: &str) -> Result<Self, crate::CellParseError> {
        match input {
            "hartree/bohr" => Ok(ForceUnits::HartreePerBohr),
            "ev/ang" => Ok(ForceUnits::ElectronVoltsPerAng),
            "n" => Ok(ForceUnits::Newton),
            _ => Err(CellParseError::Invalid),
        }
    }
}
