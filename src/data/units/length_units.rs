use std::fmt::Display;

use crate::CellParseError;

use super::ParsableUnit;

#[derive(Debug, Default, Clone, Copy)]
pub enum LengthUnit {
    Bohr,
    Meter,
    Centimeter,
    Nanometer,
    #[default]
    Ang,
}

impl Display for LengthUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LengthUnit::Bohr => f.write_str("bohr"),
            LengthUnit::Meter => f.write_str("m"),
            LengthUnit::Centimeter => f.write_str("cm"),
            LengthUnit::Nanometer => f.write_str("nm"),
            LengthUnit::Ang => f.write_str("ang"),
        }
    }
}

impl ParsableUnit for LengthUnit {
    fn parse_from_str(input: &str) -> Result<Self, CellParseError> {
        match input {
            "bohr" => Ok(LengthUnit::Bohr),
            "a0" => Ok(LengthUnit::Bohr),
            "m" => Ok(LengthUnit::Meter),
            "cm" => Ok(LengthUnit::Centimeter),
            "nm" => Ok(LengthUnit::Nanometer),
            "ang" => Ok(LengthUnit::Ang),
            _ => Err(CellParseError::Invalid),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum InvLengthUnit {
    Bohr,
    Meter,
    Nanometer,
    #[default]
    Ang,
}

impl Display for InvLengthUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvLengthUnit::Bohr => f.write_str("1/"),
            InvLengthUnit::Meter => f.write_str("1/m"),
            InvLengthUnit::Nanometer => f.write_str("1/nm"),
            InvLengthUnit::Ang => f.write_str("1/ang"),
        }
    }
}

impl ParsableUnit for InvLengthUnit {
    fn parse_from_str(input: &str) -> Result<Self, CellParseError> {
        match input {
            "1/" => Ok(InvLengthUnit::Bohr),
            "1/m" => Ok(InvLengthUnit::Meter),
            "1/nm" => Ok(InvLengthUnit::Nanometer),
            "1/ang" => Ok(InvLengthUnit::Ang),
            _ => Err(CellParseError::Invalid),
        }
    }
}
