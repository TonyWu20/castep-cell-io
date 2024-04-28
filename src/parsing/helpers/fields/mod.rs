use winnow::{ascii::space0, token::take_until, PResult, Parser};

use super::{effective_line, KeywordType};

/// When it is "keyword : value"
pub fn field_name<'s>(input: &mut &'s str) -> PResult<KeywordType<'s>> {
    take_until(0.., ":")
        .map(|s: &str| KeywordType::Field(s.trim()))
        .parse_next(input)
}

pub fn get_field_data<'s>(input: &mut &'s str) -> PResult<&'s str> {
    (":", space0, effective_line)
        .map(|(_, _, s): (&str, &str, &str)| s)
        .parse_next(input)
}
