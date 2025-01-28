use crate::parser::Rule;
use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::{Parser, Span};
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    KeywordDisplay,
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(field = "OPT_STRATEGY")]
#[pest_ast(rule(Rule::opt_strategy))]
#[pest_rule(rule=Rule::opt_strategy, keyword="OPT_STRATEGY")]
/// This parameter determines the optimization strategy used when there are multiple strategies available for the selected algorithm and they have differing costs in terms of memory usage and performance. Available options are:
/// - Speed - uses the optimization strategy which maximizes performance at the cost of additional memory usage.
/// - Default - uses the optimization strategy that best balances performance and memory usage.
/// - Memory - uses the optimization strategy which minimizes memory usage at a cost of decreased performance.
/// # Default
/// `Default`
/// # Example
/// `OPT_STRATEGY : Memory`
pub enum OptStrategy {
    #[pest_ast(inner(rule(Rule::opt_strategy_value), with(span_into_opt_strategy)))]
    Speed,
    #[default]
    Default,
    Memory,
}

fn span_into_opt_strategy(span: Span<'_>) -> OptStrategy {
    let value = span.as_str().to_lowercase();
    match value.as_str() {
        "speed" => Some(OptStrategy::Speed),
        "default" => Some(OptStrategy::Default),
        "memory" => Some(OptStrategy::Memory),
        _ => None,
    }
    .expect("Always correct from parsed value.")
}
