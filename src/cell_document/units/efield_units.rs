use std::fmt::Display;

use crate::CellParseError;

use super::{charge_units::ChargeUnits, force_units::ForceUnits, ParsableUnit};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct EFieldUnit {
    force: ForceUnits,
    charge: ChargeUnits,
}

impl EFieldUnit {
    pub fn force(&self) -> ForceUnits {
        self.force
    }

    pub fn charge(&self) -> ChargeUnits {
        self.charge
    }
}

impl Display for EFieldUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.force, self.charge)
    }
}

impl ParsableUnit for EFieldUnit {
    fn parse_from_str(input: &str) -> Result<Self, crate::CellParseError> {
        input
            .match_indices('/')
            .nth(1)
            .map(|(index, _)| {
                let (force_str, charge_str) = input.split_at(index);
                let force_unit = ForceUnits::parse_from_str(force_str)?;
                let charge_str = charge_str
                    .strip_prefix('/')
                    .ok_or(CellParseError::Invalid)?;
                let charge_unit = ChargeUnits::parse_from_str(charge_str)?;
                Ok((force_unit, charge_unit))
            })
            .ok_or(CellParseError::Invalid)?
            .map(|(force, charge)| EFieldUnit { force, charge })
    }
}
