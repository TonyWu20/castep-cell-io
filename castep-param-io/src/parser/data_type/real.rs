use pest::Span;
use pest_ast::FromPest;

use crate::parser::Rule;

#[derive(Debug, Clone, Copy, PartialEq, FromPest)]
#[pest_ast(rule(Rule::real))]
pub struct Real<'a>(#[pest_ast(outer())] pub Span<'a>);

impl<'a> Real<'a> {
    pub fn from_span(span: Span<'a>) -> Self {
        Real(span)
    }
}

impl<'a> From<Real<'a>> for f64 {
    fn from(value: Real<'a>) -> Self {
        let value = value.0.as_str();
        value.parse::<f64>().expect("Should be real number")
    }
}

#[cfg(test)]
mod test {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::parser::{data_type::real::Real, ParamParser, Rule};

    #[test]
    fn rule_real() {
        let mut tree = ParamParser::parse(Rule::real, "1.6e-5").unwrap();
        println!("{:?}", tree);
        let number = Real::from_pest(&mut tree).unwrap();
        dbg!(number);
        let mut parse = ParamParser::parse(Rule::real, "1e4").unwrap();
        let number = Real::from_pest(&mut parse).unwrap();
        dbg!(number);
        let mut parse = ParamParser::parse(Rule::real, "-1.6e-4").unwrap();
        let number = Real::from_pest(&mut parse).unwrap();
        dbg!(number);
        let mut parse = ParamParser::parse(Rule::real, "-0.5").unwrap();
        let number = Real::from_pest(&mut parse).unwrap();
        dbg!(number);
        let mut parse = ParamParser::parse(Rule::real, "0.50000000000000000").unwrap();
        let number = Real::from_pest(&mut parse).unwrap();
        dbg!(number);
        let mut parse = ParamParser::parse(Rule::real, ".5").unwrap();
        let number = Real::from_pest(&mut parse).unwrap();
        dbg!(number);
        let mut parse = ParamParser::parse(Rule::real, "-.5").unwrap();
        let number = Real::from_pest(&mut parse).unwrap();
        dbg!(number);
        let mut parse = ParamParser::parse(Rule::real, "-.5e6").unwrap();
        let number = Real::from_pest(&mut parse).unwrap();
        dbg!(number);
    }
}
