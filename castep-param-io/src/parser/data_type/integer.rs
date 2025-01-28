use std::str::FromStr;

use pest::Span;
use pest_ast::FromPest;

use crate::parser::Rule;

#[derive(Debug, Clone, Copy, PartialEq, FromPest)]
#[pest_ast(rule(Rule::pos_integer))]
/// Parse result for non-negative integer as u64
pub struct PosInteger<'a>(#[pest_ast(outer())] Span<'a>);

impl<'a> PosInteger<'a> {
    pub fn new(span: Span<'a>) -> Self {
        Self(span)
    }
}

impl TryFrom<PosInteger<'_>> for u64 {
    type Error = <u64 as FromStr>::Err;

    fn try_from(value: PosInteger) -> Result<Self, Self::Error> {
        value.0.as_str().parse::<u64>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, FromPest)]
#[pest_ast(rule(Rule::integer))]
/// Parse result for integer as i64
pub struct Integer<'a>(#[pest_ast(outer())] Span<'a>);

impl<'a> Integer<'a> {
    pub fn new(span: Span<'a>) -> Self {
        Self(span)
    }
}

impl TryFrom<Integer<'_>> for i64 {
    type Error = <i64 as FromStr>::Err;

    fn try_from(value: Integer) -> Result<Self, Self::Error> {
        value.0.as_str().parse::<i64>()
    }
}
