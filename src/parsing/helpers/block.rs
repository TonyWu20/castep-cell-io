use winnow::{
    ascii::{line_ending, space1, till_line_ending, Caseless},
    combinator::{delimited, terminated},
    token::take_until,
    PResult, Parser,
};

/// Go to block name
/// essential for matching the next data is block type or field type.
pub fn strip_to_block_name<'s>(input: &mut &'s str) -> PResult<&'s str> {
    delimited(
        terminated(Caseless("%block"), space1),
        till_line_ending,
        line_ending,
    )
    .parse_next(input)
}
/// Get contents in block
fn contents_in_block<'s>(input: &mut &'s str) -> PResult<&'s str> {
    take_until(0.., '%').parse_next(input)
}
/// Move out of the block ending line
fn end_of_block<'s>(input: &mut &'s str) -> PResult<(&'s str, &'s str, &'s str)> {
    (till_line_ending, line_ending, line_ending).parse_next(input)
}

/// When the block name is identified, the remaining `input`
/// is directly the lines for data and the ending line of the block
/// Returns the lines and throw away the ending to move out of the
/// block in `input`
pub fn get_block_data<'s>(input: &mut &'s str) -> PResult<&'s str> {
    terminated(contents_in_block, end_of_block).parse_next(input)
}
