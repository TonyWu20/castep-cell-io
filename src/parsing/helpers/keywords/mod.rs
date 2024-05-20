use winnow::{combinator::rest, PResult, Parser};

use crate::{
    data::LengthUnit,
    keywords::{DocumentSections, KeywordType},
    CellParseError,
};

pub(crate) mod ionic_positions;
pub(crate) mod lattice;

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
