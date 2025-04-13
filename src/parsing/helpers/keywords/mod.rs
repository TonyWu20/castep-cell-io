use winnow::{token::rest, ModalResult, Parser};

use crate::keywords::{DocumentSections, KeywordType};

pub(crate) mod ionic_positions;
pub(crate) mod kpoint;
pub(crate) mod lattice;
pub(crate) mod species;
// TODO: More sections

pub fn any_block<'s>(input: &mut &'s str) -> ModalResult<DocumentSections<'s>> {
    rest.map(|s: &str| DocumentSections::Misc(KeywordType::Block(s)))
        .parse_next(input)
}

pub fn any_field<'s>(input: &mut &'s str) -> ModalResult<DocumentSections<'s>> {
    rest.map(|s: &str| DocumentSections::Misc(KeywordType::Field(s)))
        .parse_next(input)
}
