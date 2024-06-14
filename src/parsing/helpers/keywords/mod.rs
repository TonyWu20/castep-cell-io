use winnow::{combinator::rest, PResult, Parser};

use crate::{
    keywords::{DocumentSections, KeywordType},
    CellParseError, InvLengthUnit, LengthUnit,
};

pub(crate) mod ionic_positions;
pub(crate) mod kpoint;
pub(crate) mod lattice;
pub(crate) mod species;

pub fn any_block<'s>(input: &mut &'s str) -> PResult<DocumentSections<'s>> {
    rest.map(|s: &str| DocumentSections::Misc(KeywordType::Block(s)))
        .parse_next(input)
}

fn length_unit(input: &str) -> Result<LengthUnit, CellParseError> {
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

fn inv_length_unit(input: &str) -> Result<InvLengthUnit, CellParseError> {
    match input {
        "1/" => Ok(InvLengthUnit::Bohr),
        "1/m" => Ok(InvLengthUnit::Meter),
        "1/nm" => Ok(InvLengthUnit::Nanometer),
        "1/ang" => Ok(InvLengthUnit::Ang),
        _ => Err(CellParseError::Invalid),
    }
}
