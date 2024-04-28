use winnow::{combinator::rest, PResult, Parser};

use crate::keywords::{DocumentSections, KeywordType};

pub(crate) mod ionic_positions;
pub(crate) mod lattice;

pub fn any_block<'s>(input: &mut &'s str) -> PResult<DocumentSections<'s>> {
    rest.map(|s: &str| DocumentSections::Misc(KeywordType::Block(s)))
        .parse_next(input)
}
