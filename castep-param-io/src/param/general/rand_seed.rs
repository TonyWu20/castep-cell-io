use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::parser::{data_type::Integer, Rule};

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
    BuildFromPairs,
)]
#[keyword_display(field="RAND_SEED",from=i64,value=i64)]
#[pest_ast(rule(Rule::rand_seed))]
#[pest_rule(rule=Rule::rand_seed, keyword="RAND_SEED")]
pub struct RandSeed(
    #[pest_ast(inner(with(Integer::new), with(i64::try_from), with(Result::unwrap)))] i64,
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
