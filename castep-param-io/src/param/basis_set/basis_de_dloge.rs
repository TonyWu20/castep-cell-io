use crate::parser::{data_type::Real, Rule};
use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    PartialOrd,
    Default,
    KeywordDisplay,
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(from=f64, value=f64, field="BASIS_DE_DLOGE")]
#[pest_ast(rule(Rule::basis_de_dloge))]
#[pest_rule(rule=Rule::basis_de_dloge,keyword="BASIS_DE_DLOGE")]
pub struct BasisDeDloge(
    #[pest_ast(inner(rule(Rule::real), with(Real::from_span), with(f64::from)))] f64,
);
