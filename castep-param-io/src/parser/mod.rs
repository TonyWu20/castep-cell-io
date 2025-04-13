use std::fmt::Display;

use pest::Span;
use pest_ast::FromPest;
use pest_derive::Parser;

pub mod data_type;
mod general;

#[derive(Parser)]
#[grammar = "parser/param.pest"]
pub struct ParamParser;

pub fn span_into_str(span: Span) -> &str {
    span.as_str()
}

#[derive(Debug, Clone, PartialEq, FromPest)]
#[pest_ast(rule(Rule::param_file))]
pub struct ParamFile<'a> {
    items: Vec<ParamItems<'a>>,
}

impl<'a> ParamFile<'a> {
    pub fn items(&self) -> &[ParamItems<'a>] {
        &self.items
    }
}

#[derive(Debug, Clone, Copy, PartialEq, FromPest)]
#[pest_ast(rule(Rule::kv_pair))]
pub struct KVPair<'a> {
    #[pest_ast(inner(rule(Rule::keyword), with(span_into_str)))]
    keyword: &'a str,
    #[pest_ast(inner(rule(Rule::value)))]
    value: Span<'a>,
}

fn span_into_stop(_span: Span<'_>) -> ParamItems<'_> {
    ParamItems::Stop
}

#[derive(Debug, Clone, Copy, PartialEq, FromPest)]
#[pest_ast(rule(Rule::param_item))]
pub enum ParamItems<'a> {
    Pairs(KVPair<'a>),
    #[pest_ast(inner(rule(Rule::stop), with(span_into_stop)))]
    Stop,
}

impl ParamItems<'_> {
    pub fn keyword(&self) -> String {
        match self {
            ParamItems::Pairs(kvpair) => kvpair.keyword().into(),
            ParamItems::Stop => "stop".to_string(),
        }
    }
}

impl PartialOrd for KVPair<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.keyword.partial_cmp(other.keyword)
    }
}

impl<'a> KVPair<'a> {
    pub fn keyword(&self) -> &str {
        self.keyword
    }

    pub fn value(&self) -> Span<'a> {
        self.value
    }
}

impl Display for KVPair<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {}", self.keyword, self.value.as_str())
    }
}

impl Display for ParamItems<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParamItems::Pairs(kvpair) => write!(f, "{}", kvpair),
            ParamItems::Stop => f.write_str("stop"),
        }
    }
}

pub trait ConsumeKVPairs<'a> {
    type Item;
    fn find_from_pairs(items: &'a [ParamItems<'a>]) -> Option<Self::Item>;
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;

    use from_pest::FromPest;
    use pest::Parser;

    use crate::{param::General, parser::ConsumeKVPairs};

    use super::{ParamFile, ParamParser, Rule};

    #[test]
    fn test_param() {
        let param = read_to_string("NP22_single_0.param").unwrap();
        let mut parse = ParamParser::parse(Rule::param_file, &param).unwrap();
        dbg!(&parse);
        let parsed_param = ParamFile::from_pest(&mut parse).unwrap();
        let general_section = General::find_from_pairs(parsed_param.items());
        println!("{:?}", parsed_param);
        println!("{}", general_section.unwrap());
    }
}
