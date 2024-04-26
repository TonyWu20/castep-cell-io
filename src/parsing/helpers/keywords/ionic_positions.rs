use winnow::{ascii::Caseless, combinator::alt, PResult, Parser};

use crate::keywords::{DocumentSections, PositionsKeywords};

fn assign_positions_frac<'s>(input: &mut &'s str) -> PResult<DocumentSections> {
    Caseless("positions_frac")
        .map(|_| DocumentSections::IonicPositions(PositionsKeywords::POSITIONS_FRAC))
        .parse_next(input)
}

fn assign_positions_abs<'s>(input: &mut &'s str) -> PResult<DocumentSections> {
    Caseless("positions_abs")
        .map(|_| DocumentSections::IonicPositions(PositionsKeywords::POSITIONS_ABS))
        .parse_next(input)
}

pub fn assign_positions_type<'s>(input: &mut &'s str) -> PResult<DocumentSections> {
    alt((assign_positions_frac, assign_positions_abs)).parse_next(input)
}
