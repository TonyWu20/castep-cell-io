use castep_param_derive::KeywordDisplay;
use from_pest::FromPest;
use pest::{Parser, Span};
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::parser::{span_into_str, ConsumeKVPairs, ParamParser, Rule};

#[derive(
    Debug,
    Clone,
    Hash,
    Serialize,
    Deserialize,
    KeywordDisplay,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromPest,
)]
#[keyword_display(specified_fields = true)]
#[pest_ast(rule(Rule::continue_reuse))]
pub enum ContinueReuse {
    #[keyword_display(field = "CONTINUATION")]
    Continuation(Continuation),
    #[keyword_display(field = "REUSE")]
    Reuse(Reuse),
}

impl<'a> ConsumeKVPairs<'a> for ContinueReuse {
    type Item = Self;

    fn find_from_pairs(pairs: &'a [crate::parser::KVPair<'a>]) -> Option<Self::Item> {
        let cont_id = pairs
            .iter()
            .position(|p| p.keyword().to_lowercase() == "continuation");
        let reuse_id = pairs
            .iter()
            .position(|p| p.keyword().to_lowercase() == "reuse");
        let to_use = match (cont_id, reuse_id) {
            (None, None) => None,
            (None, Some(n)) | (Some(n), None) => Some(n),
            (Some(c), Some(r)) => {
                if c > r {
                    Some(c)
                } else {
                    Some(r)
                }
            }
        };
        to_use.map(|id| {
            let kvpair = pairs[id].to_string();
            dbg!(&kvpair);
            let mut parse = ParamParser::parse(Rule::continue_reuse, &kvpair).unwrap();
            Self::from_pest(&mut parse).unwrap()
        })
    }
}

impl Default for ContinueReuse {
    fn default() -> Self {
        Self::Continuation(Continuation::Default)
    }
}

impl From<Continuation> for ContinueReuse {
    fn from(value: Continuation) -> Self {
        ContinueReuse::Continuation(value)
    }
}

impl From<Reuse> for ContinueReuse {
    fn from(value: Reuse) -> Self {
        ContinueReuse::Reuse(value)
    }
}

#[derive(
    Debug,
    Clone,
    Default,
    Hash,
    Serialize,
    Deserialize,
    KeywordDisplay,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromPest,
)]
#[keyword_display(field = "CONTINUATION")]
#[pest_ast(rule(Rule::continuation))]
pub enum Continuation {
    #[pest_ast(inner(
        rule(Rule::continuation_default),
        with(span_into_str),
        with(Continuation::from_str_default),
        with(Option::unwrap)
    ))]
    #[default]
    Default,
    File(
        #[pest_ast(inner(rule(Rule::continuation_file), with(span_into_str), with(String::from)))]
        String,
    ),
}

impl<'a> From<Span<'a>> for Continuation {
    fn from(value: Span<'a>) -> Self {
        let mut parse = ParamParser::parse(Rule::continuation, value.as_str()).unwrap();
        Self::from_pest(&mut parse).unwrap()
    }
}

impl Continuation {
    fn from_str_default(input: &str) -> Option<Self> {
        if input.to_lowercase() == "default" {
            Some(Self::Default)
        } else {
            None
        }
    }

    pub fn as_file(&self) -> Option<&String> {
        if let Self::File(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

#[derive(
    Debug,
    Clone,
    Default,
    Hash,
    Serialize,
    Deserialize,
    KeywordDisplay,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromPest,
)]
#[keyword_display(field = "REUSE")]
#[pest_ast(rule(Rule::reuse))]
pub enum Reuse {
    #[pest_ast(inner(
        rule(Rule::reuse_default),
        with(span_into_str),
        with(Reuse::from_str_default),
        with(Option::unwrap)
    ))]
    #[default]
    Default,
    File(
        #[pest_ast(inner(rule(Rule::reuse_file), with(span_into_str), with(String::from)))] String,
    ),
}

impl Reuse {
    fn from_str_default(input: &str) -> Option<Self> {
        if input.to_lowercase() == "default" {
            Some(Self::Default)
        } else {
            None
        }
    }

    pub fn as_file(&self) -> Option<&String> {
        if let Self::File(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::{
        param::{ContinueReuse, KeywordDisplay, Reuse},
        parser::{ParamParser, Rule},
    };

    use super::Continuation;

    #[test]
    fn continuation_parse() {
        let input = "continuation : default\r\n";
        let mut parse = ParamParser::parse(Rule::continuation, input).unwrap();
        let cont = Continuation::from_pest(&mut parse);
        assert!(cont.is_ok());
        let input = "continuation : seed.castep\n";
        let mut parse = ParamParser::parse(Rule::continuation, input).unwrap();
        let cont = Continuation::from_pest(&mut parse);
        assert!(cont.is_ok());
        assert!(cont.unwrap().as_file().is_some_and(|x| x == "seed.castep"));
    }
    #[test]
    fn reuse_parse() {
        let input = "reuse : default\r\n";
        let mut parse = ParamParser::parse(Rule::reuse, input).unwrap();
        let cont = Reuse::from_pest(&mut parse);
        assert!(cont.is_ok());
        let input = "reuse : seed.castep\n";
        let mut parse = ParamParser::parse(Rule::reuse, input).unwrap();
        let cont = Reuse::from_pest(&mut parse);
        assert!(cont.is_ok());
        assert!(cont.unwrap().as_file().is_some_and(|x| x == "seed.castep"));
    }
    #[test]
    fn cr_parse() {
        let input = "continuation : default\r\n";
        let mut parse = ParamParser::parse(Rule::continue_reuse, input).unwrap();
        let cont = ContinueReuse::from_pest(&mut parse);
        dbg!(&cont);
        assert!(cont.is_ok());
        let input = "continuation : seed.castep\n";
        let mut parse = ParamParser::parse(Rule::continue_reuse, input).unwrap();
        let cont = ContinueReuse::from_pest(&mut parse);
        dbg!(&cont);
        assert!(cont.is_ok());
        let input = "reuse : default\r\n";
        let mut parse = ParamParser::parse(Rule::continue_reuse, input).unwrap();
        dbg!(&parse);
        let cont = ContinueReuse::from_pest(&mut parse);
        dbg!(&cont);
        assert!(cont.is_ok());
        let input = "reuse : NP22_single_0";
        let mut parse = ParamParser::parse(Rule::continue_reuse, input).unwrap();
        dbg!(&parse);
        let cont = ContinueReuse::from_pest(&mut parse);
        dbg!(&cont);
        assert!(cont.is_ok());
        println!("{}", cont.unwrap().output());
    }
}
