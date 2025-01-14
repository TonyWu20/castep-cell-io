
use castep_param_derive::KeywordDisplay;
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
)]
#[keyword_display(field = "OPT_STRATEGY")]
/// This parameter determines the optimization strategy used when there are multiple strategies available for the selected algorithm and they have differing costs in terms of memory usage and performance. Available options are:
/// - Speed - uses the optimization strategy which maximizes performance at the cost of additional memory usage.
/// - Default - uses the optimization strategy that best balances performance and memory usage.
/// - Memory - uses the optimization strategy which minimizes memory usage at a cost of decreased performance.
/// # Default
/// `Default`
/// # Example
/// `OPT_STRATEGY : Memory`
pub enum OptStrategy {
    Speed,
    #[default]
    Default,
    Memory,
}
