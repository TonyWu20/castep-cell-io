use pest::Span;
use pest_ast::FromPest;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/param.pest"]
pub struct ParamParser;

fn span_into_str(span: Span) -> &str {
    dbg!(span.as_str())
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, FromPest)]
#[pest_ast(rule(Rule::real))]
pub struct Real {
    #[pest_ast(outer(with(span_into_str), with(str::parse::<f64>), with(Result::unwrap)))]
    pub value: f64,
}

#[cfg(test)]
mod test {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::parser::Real;

    use super::{ParamParser, Rule};
    #[test]
    fn parse_real() {
        let mut tree = ParamParser::parse(Rule::real, "1.6e-5").unwrap();
        println!("{:?}", tree);
        let number = Real::from_pest(&mut tree);
        match number {
            Ok(e) => println!("{e:?}"),
            Err(e) => println!("{e}"),
        }
        // let parse = ParamParser::parse(Rule::real, "1e4").unwrap();
        // parse.into_iter().for_each(|p| println!("{p}"));
        // let parse = ParamParser::parse(Rule::real, "-1.6e-4");
        // println!("{:?}", parse);
        let mut parse = ParamParser::parse(Rule::real, "-0.5").unwrap();
        dbg!(Real::from_pest(&mut parse).unwrap());
        // println!("{:?}", parse);
        // let parse = ParamParser::parse(Rule::real, "-.5");
        // println!("{:?}", parse);
    }
}
