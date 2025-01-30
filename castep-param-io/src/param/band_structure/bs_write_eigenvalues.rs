use crate::parser::{data_type::Logical, Rule};
use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Serialize,
    Deserialize,
    KeywordDisplay,
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(field="BS_WRITE_EIGENVALUES", from=bool,value=bool)]
#[pest_ast(rule(Rule::bs_write_eigenvalues))]
#[pest_rule(rule=Rule::bs_write_eigenvalues,keyword="BS_WRITE_EIGENVALUES")]
pub struct BSWriteEigenvalues(
    #[pest_ast(inner(rule(Rule::logical), with(Logical::from), with(Logical::into)))] bool,
);
