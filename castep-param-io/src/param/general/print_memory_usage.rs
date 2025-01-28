use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::parser::{data_type::Logical, Rule};

#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    KeywordDisplay,
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(field="PRINT_MEMORY_USAGE",from=bool,value=bool, default_value=true)]
#[pest_ast(rule(Rule::print_memory_usage))]
#[pest_rule(rule=Rule::print_memory_usage, keyword="PRINT_MEMORY_USAGE")]
pub struct PrintMemoryUsage(
    #[pest_ast(inner(rule(Rule::logical), with(Logical::from), with(Logical::into)))] bool,
);
