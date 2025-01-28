use std::fmt::Display;

use pest::Span;
use pest_ast::FromPest;
use pest_derive::Parser;

use crate::param::Stop;

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
    stop: Option<Stop>,
}

impl<'a> ParamFile<'a> {
    pub fn pairs(&self) -> &[KVPair<'a>] {
        &self.pairs
    }

    pub fn pairs_mut(&mut self) -> &mut Vec<KVPair<'a>> {
        &mut self.pairs
    }

    pub fn stop(&self) -> Option<Stop> {
        self.stop
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

    /// Sort parsed keyword pairs according to the order of fields in `CastepParam`
    pub fn ordered_copy(&self) -> Self {
        // Use enumerate to mark the current order with indices.
        todo!();
    }
}

impl Display for KVPair<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {}", self.keyword, self.value.as_str())
    }
}

pub trait ConsumeKVPairs<'a> {
    type Item;
    fn find_from_pairs(pairs: &'a [KVPair<'a>]) -> Option<Self::Item>;
}

pub trait FromParamFile {
    type Item: Sized;
    fn build_from_parsed(_parsed_pairs: &[KVPair<'_>]) -> Self::Item {
        todo!()
    }
    fn build_from_file(_file: &ParamFile<'_>) -> Self::Item {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;

    use from_pest::FromPest;
    use pest::Parser;

    use crate::{param::General, parser::FromParamFile};

    use super::{ParamFile, ParamParser, Rule};

    #[test]
    fn test_param() {
        let param = read_to_string("NP22_single_0.param").unwrap();
        let mut parse = ParamParser::parse(Rule::param_file, &param).unwrap();
        dbg!(&parse);
        let parsed_param = ParamFile::from_pest(&mut parse).unwrap();
        let general_section = General::build_from_file(&parsed_param);
        println!("{}", general_section);
    }
}
