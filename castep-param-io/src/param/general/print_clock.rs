use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::parser::{data_type::Logical, Rule};

#[derive(
    Debug,
    Hash,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    KeywordDisplay,
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(field="PRINT_CLOCK",from=bool,value=bool, default_value=true)]
#[pest_ast(rule(Rule::print_clock))]
#[pest_rule(rule=Rule::print_clock, keyword="PRINT_CLOCK")]
pub struct PrintClock(
    #[pest_ast(inner(rule(Rule::logical), with(Logical::from), with(Logical::into)))] bool,
);
