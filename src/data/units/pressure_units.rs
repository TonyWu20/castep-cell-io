use std::fmt::Display;

use crate::CellParseError;

use super::ParsableUnit;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum PressureUnits {
    HartreePerBohr,
    ElectronVoltsPerAng,
    Pascal,
    Megapascal,
    #[default]
    Gigapascal,
    Atmosphere,
    Bar,
    Megabar,
}

impl Display for PressureUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PressureUnits::HartreePerBohr => f.write_str("hartree/bohr**3"),
            PressureUnits::ElectronVoltsPerAng => f.write_str("ev/ang**3"),
            PressureUnits::Pascal => f.write_str("pa"),
            PressureUnits::Megapascal => f.write_str("mpa"),
            PressureUnits::Gigapascal => f.write_str("gpa"),
            PressureUnits::Atmosphere => f.write_str("atm"),
            PressureUnits::Bar => f.write_str("bar"),
            PressureUnits::Megabar => f.write_str("mbar"),
        }
    }
}

impl ParsableUnit for PressureUnits {
    fn parse_from_str(input: &str) -> Result<Self, CellParseError> {
        match input {
            "hartree/bohr**3" => Ok(Self::HartreePerBohr),
            "ev/ang**3" => Ok(Self::ElectronVoltsPerAng),
            "pa" => Ok(Self::Pascal),
            "mpa" => Ok(Self::Megapascal),
            "gpa" => Ok(Self::Gigapascal),
            "atm" => Ok(Self::Atmosphere),
            "bar" => Ok(Self::Bar),
            "mbar" => Ok(Self::Megabar),
            _ => Err(CellParseError::Invalid),
        }
    }
}
