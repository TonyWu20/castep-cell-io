use castep_param_derive::BuildFromPairs;
use from_pest::FromPest;
use pest::{Parser, Span};
use std::fmt::Display;

use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::{param::KeywordDisplay, parser::Rule};

#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    FromPest,
    BuildFromPairs,
)]
/// This keyword specifies the units in which time will be reported.
/// # Example
/// `TIME_UNIT : aut`
#[pest_ast(rule(Rule::time_unit))]
#[pest_rule(rule=Rule::time_unit,keyword="TIME_UNIT")]
pub enum TimeUnit {
    #[pest_ast(inner(rule(Rule::time_unit), with(from_span)))]
    AtomicUnitOfTime,
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
    #[default]
    Picosecond,
    Femtosecond,
}

fn from_span(span: Span<'_>) -> TimeUnit {
    let input = span.as_str();
    match input.to_lowercase().as_str() {
        "aut" => Some(TimeUnit::AtomicUnitOfTime),
        "s" => Some(TimeUnit::Second),
        "ms" => Some(TimeUnit::Millisecond),
        "mus" => Some(TimeUnit::Microsecond),
        "ns" => Some(TimeUnit::Nanosecond),
        "ps" => Some(TimeUnit::Picosecond),
        "fs" => Some(TimeUnit::Femtosecond),
        _ => None,
    }
    .expect("Always correct")
}

impl Display for TimeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeUnit::AtomicUnitOfTime => f.write_str("aut"),
            TimeUnit::Second => f.write_str("s"),
            TimeUnit::Millisecond => f.write_str("ms"),
            TimeUnit::Microsecond => f.write_str("mus"),
            TimeUnit::Nanosecond => f.write_str("ns"),
            TimeUnit::Picosecond => f.write_str("ps"),
            TimeUnit::Femtosecond => f.write_str("fs"),
        }
    }
}

impl KeywordDisplay for TimeUnit {
    fn field(&self) -> String {
        "TIME_UNIT".to_string()
    }
}
