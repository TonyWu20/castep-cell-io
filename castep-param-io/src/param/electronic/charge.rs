use crate::parser::{data_type::Real, Rule};
use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

/// Description
/// This keyword specifies the total charge of the system. It can be used instead of `NELECTRONS`.
/// It is not possible to specify the `NELECTRONS`, `NUP`, or `NDOWN` keywords when the `CHARGE` keyword is specified.
/// Default
/// 0
/// Example
/// CHARGE : 3
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    PartialOrd,
    KeywordDisplay,
    FromPest,
    BuildFromPairs,
)]
#[pest_ast(rule(Rule::charge))]
#[pest_rule(rule=Rule::charge,keyword="CHARGE")]
#[keyword_display(field="CHARGE", from=f64, value=f64)]
pub struct Charge(#[pest_ast(inner(rule(Rule::real), with(Real::from_span), with(f64::from)))] f64);
