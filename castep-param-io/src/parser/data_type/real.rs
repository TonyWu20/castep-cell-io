use pest_ast::FromPest;

use crate::parser::{span_into_str, Rule};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, FromPest)]
#[pest_ast(rule(Rule::real))]
pub struct Real(
    #[pest_ast(outer(with(span_into_str), with(str::parse::<f64>), with(Result::unwrap)))] pub f64,
);

impl std::ops::Deref for Real {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
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
