use winnow::{ascii::space1, combinator::terminated, PResult, Parser};

mod lattice;
mod positions;

pub fn block_prefix<'s>(input: &mut &'s str) -> PResult<&'s str> {
    terminated("%BLOCK", space1).parse_next(input)
}
