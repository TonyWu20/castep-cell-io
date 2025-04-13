use crate::parser::{data_type::Logical, Rule};
use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

/// This keyword determines whether a fixed number of plane waves (fixed size
/// basis : TRUE) or a fixed plane wave cutoff energy
/// (fixed quality basis : FALSE) will be used when doing a variable cell
/// calculation.
/// This setting affects geometry optimization with variable cell parameters
/// and molecular dynamics with NPT or NPH ensembles.
/// # Note
/// You should turn off finite basis set correction when using a
/// fixed size basis (see FINITE_BASIS_CORR).
/// # Default
/// `FALSE`
/// # Example
/// `FIXED_NPW : TRUE`
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    KeywordDisplay,
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(field="FIXED_NPW", from=bool,value=bool)]
#[pest_ast(rule(Rule::fixed_npw))]
#[pest_rule(rule=Rule::fixed_npw,keyword="FIXED_NPW")]
pub struct FixedNPW(
    #[pest_ast(inner(rule(Rule::logical), with(Logical::from), with(Logical::into)))] bool,
);
