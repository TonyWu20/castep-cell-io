use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::parser::{data_type::Integer, Rule};

/// This keyword specifies the maximum run time for the job, in seconds. If the RUN_TIME is greater than zero, the job will exit cleanly before the specified time has elapsed, leaving as little unused time as possible.
/// If RUN_TIME is less than or equal to zero, no time limit will be imposed on the run.
/// # Default
/// 0
/// # Example
/// `RUN_TIME : 360`
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
    KeywordDisplay,
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(field="RUN_TIME",from=i64,value=i64)]
#[pest_ast(rule(Rule::run_time))]
#[pest_rule(rule=Rule::run_time,keyword="RUN_TIME")]
pub struct RunTime(
    #[pest_ast(inner(with(Integer::new), with(i64::try_from), with(Result::unwrap)))] i64,
);
