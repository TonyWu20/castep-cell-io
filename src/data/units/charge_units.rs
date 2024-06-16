use std::fmt::Display;

use crate::CellParseError;

use super::ParsableUnit;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChargeUnits {
    #[default]
    ElementaryCharge,
    Coulomb,
}

impl Display for ChargeUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChargeUnits::ElementaryCharge => f.write_str("e"),
            ChargeUnits::Coulomb => f.write_str("c"),
        }
    }
}

impl ParsableUnit for ChargeUnits {
    fn parse_from_str(input: &str) -> Result<Self, crate::CellParseError> {
        match input {
            "e" => Ok(Self::ElementaryCharge),
            "c" => Ok(Self::Coulomb),
            _ => Err(CellParseError::Invalid),
        }
    }
}
