use from_pest::FromPest;
use pest::Parser;
use std::{fmt::Display, marker::PhantomData};

use castep_param_derive::BuildFromPairs;
use pest::Span;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::{param::KeywordDisplay, parser::Rule};

#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    Serialize,
    Deserialize,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromPest,
    BuildFromPairs,
)]
/// This keyword, if present, will cause the current run to be aborted as if RUN_TIME had been exceeded.
///
/// CASTEP checks the contents of the input file periodically during a run. This allows you to modify certain parameters and also to terminate the run early.
///
/// This keyword is valid only when the input file is reread. It is ignored if it is present at the start of a run.
/// # Example
/// `STOP`
#[pest_ast(rule(Rule::param_item))]
#[pest_rule(rule=Rule::param_item, keyword= "STOP")]
pub struct Stop(#[pest_ast(inner(rule(Rule::stop), with(span_into_phantom)))] PhantomData<String>);

fn span_into_phantom(_span: Span<'_>) -> PhantomData<String> {
    PhantomData
}

impl Display for Stop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("")
    }
}

impl KeywordDisplay for Stop {
    fn field(&self) -> String {
        "STOP".to_string()
    }
    fn output(&self) -> String {
        self.field()
    }
}

#[cfg(test)]
mod test {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::parser::{ParamParser, Rule};

    use super::Stop;

    #[test]
    fn stop_parse() {
        let input = "STOP";
        let mut parse = ParamParser::parse(Rule::param_item, input).unwrap();
        let stop = Stop::from_pest(&mut parse).unwrap();
        dbg!(stop);
    }
}
