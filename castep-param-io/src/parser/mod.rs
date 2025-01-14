use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/param.pest"]
pub struct RealParser;

#[cfg(test)]
mod test {
    use pest::Parser;

    use super::{RealParser, Rule};
    #[test]
    fn parse_real() {
        let parse = RealParser::parse(Rule::real, "-0.5");
        println!("{:?}", parse);
        let parse = RealParser::parse(Rule::real, "-.5");
        println!("{:?}", parse);
    }
}
