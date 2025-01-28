use std::{fmt::Display, marker::PhantomData};

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
)]
/// This keyword, if present, will cause the current run to be aborted as if RUN_TIME had been exceeded.
///
/// CASTEP checks the contents of the input file periodically during a run. This allows you to modify certain parameters and also to terminate the run early.
///
/// This keyword is valid only when the input file is reread. It is ignored if it is present at the start of a run.
/// # Example
/// `STOP`
#[pest_ast(rule(Rule::kv_pair))]
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
