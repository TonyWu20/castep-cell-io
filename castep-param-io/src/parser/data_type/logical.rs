use pest::Span;
use pest_ast::FromPest;

use crate::parser::Rule;
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPest)]
#[pest_ast(rule(Rule::logical))]
pub struct Logical<'a>(#[pest_ast(outer())] Span<'a>);

impl From<Logical<'_>> for bool {
    fn from(value: Logical<'_>) -> Self {
        let expr = value.0.as_str().to_lowercase();
        expr.parse::<bool>()
            .expect("Always correct from parsed result.")
    }
}

impl<'a> From<Span<'a>> for Logical<'a> {
    fn from(value: Span<'a>) -> Self {
        Self(value)
    }
}

impl<'a> Logical<'a> {
    pub fn new(span: Span<'a>) -> Self {
        Self(span)
    }
}
