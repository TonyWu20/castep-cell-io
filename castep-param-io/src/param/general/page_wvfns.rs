use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::parser::{data_type::Integer, Rule};

///This keyword controls the paging of wavefunctions to disk in order to save memory. Available options are:
/// - > 0 - all wavefunctions requiring more memory than this value in megabytes will be paged to disk.
/// - 0 - no paging will be performed.
/// - < 0 - all wavefunctions will be paged to disk.
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
#[keyword_display(field="PAGE_WVFNS",from=i64,value=i64)]
#[pest_ast(rule(Rule::page_wvfns))]
#[pest_rule(rule=Rule::page_wvfns, keyword="PAGE_WVFNS")]
pub struct PageWvfns(
    #[pest_ast(inner(with(Integer::new), with(i64::try_from), with(Result::unwrap)))] i64,
);

impl From<i32> for PageWvfns {
    fn from(value: i32) -> Self {
        Self(value as i64)
    }
}

impl From<i16> for PageWvfns {
    fn from(value: i16) -> Self {
        Self(value as i64)
    }
}

impl From<i8> for PageWvfns {
    fn from(value: i8) -> Self {
        Self(value as i64)
    }
}
