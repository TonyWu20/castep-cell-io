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
    pairs: Vec<KVPair<'a>>,
}

impl<'a> ParamFile<'a> {
    pub fn pairs(&self) -> &[KVPair<'a>] {
        &self.pairs
    }

    pub fn pairs_mut(&mut self) -> &mut Vec<KVPair<'a>> {
        &mut self.pairs
    }
}

#[derive(Debug, Clone, Copy, PartialEq, FromPest)]
#[pest_ast(rule(Rule::kv_pair))]
pub struct KVPair<'a> {
    #[pest_ast(inner(rule(Rule::keyword)))]
    keyword: Span<'a>,
    #[pest_ast(inner(rule(Rule::value)))]
    value: Span<'a>,
}

impl<'a> KVPair<'a> {
    pub fn keyword(&self) -> Span<'a> {
        self.keyword
    }

    pub fn value(&self) -> Span<'a> {
        self.value
    }
}

impl Display for KVPair<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {}", self.keyword.as_str(), self.value.as_str())
    }
}

pub trait ConsumeKVPairs<'a> {
    type Item;
    fn find_from_pairs(pairs: &'a [KVPair<'a>]) -> Option<Self::Item>;
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;

    use from_pest::FromPest;
    use pest::Parser;

    use crate::param::General;

    use super::{ParamFile, ParamParser, Rule};

    #[test]
    fn test_param() {
        let param = read_to_string("NP22_single_0.param").unwrap();
        let mut parse = ParamParser::parse(Rule::param_file, &param).unwrap();
        let mut parsed_param = ParamFile::from_pest(&mut parse).unwrap();
        let general_section = General::build_from_parsed(parsed_param.pairs_mut());
        println!("{}", general_section);
    }
}
