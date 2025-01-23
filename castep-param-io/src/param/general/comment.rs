use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::parser::{span_into_str, Rule};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    KeywordDisplay,
    Default,
    Serialize,
    Deserialize,
    Hash,
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(field="COMMENT", from=String, borrowed_value=str)]
#[pest_ast(rule(Rule::param_comment))]
#[pest_rule(rule=Rule::param_comment, keyword="COMMENT")]
pub struct Comment(#[pest_ast(inner(with(span_into_str), with(String::from)))] String);

#[cfg(test)]
mod test {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::parser::{ParamParser, Rule};

    use super::Comment;

    #[test]
    fn comment_parse() {
        let input = "COMMENT : Calculation from Rhino 5.999\n";
        let mut parse = ParamParser::parse(Rule::param_comment, input).unwrap();
        let comment = Comment::from_pest(&mut parse).unwrap();
        dbg!(comment);
    }
}
