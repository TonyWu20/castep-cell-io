use castep_param_derive::KeywordDisplay;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::parser::{span_into_str, Rule};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Default,
    Hash,
    KeywordDisplay,
    FromPest,
)]
#[keyword_display(field="RAND_SEED",from=i64,value=i64)]
#[pest_ast(rule(Rule::rand_seed))]
pub struct RandSeed(
    #[pest_ast(inner(with(span_into_str), with(str::parse::<i64>), with(Result::unwrap))) ] i64,
);

#[cfg(test)]
mod test {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::{
        param::RandSeed,
        parser::{ParamParser, Rule},
    };

    #[test]
    fn parse_randseed() {
        let input = "rand_seed : 9";
        let mut parse = ParamParser::parse(Rule::rand_seed, input).unwrap();
        println!("{:?}", parse);
        let rand_seed = RandSeed::from_pest(&mut parse).unwrap();
        println!("{:?}", rand_seed);
    }
}
