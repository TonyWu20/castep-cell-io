use from_pest::FromPest;
use pest::Parser;
use std::fmt::{Display, Write};

use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use pest::Span;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::parser::Rule;

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    Hash,
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
#[keyword_display(field = "CHARGE_UNIT", direct_display = false)]
#[pest_ast(rule(Rule::charge_unit))]
#[pest_rule(rule=Rule::charge_unit, keyword="CHARGE_UNIT")]
pub enum ChargeUnit {
    #[default]
    #[pest_ast(inner(rule(Rule::charge_unit_value), with(span_into_charge_unit)))]
    E,
    C,
}

fn span_into_charge_unit(span: Span<'_>) -> ChargeUnit {
    let value = span.as_str().to_lowercase();
    let matched = match value.as_str() {
        "e" => Some(ChargeUnit::E),
        "c" => Some(ChargeUnit::C),
        _ => None,
    };
    matched.unwrap()
}

impl Display for ChargeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChargeUnit::E => f.write_char('e'),
            ChargeUnit::C => f.write_char('c'),
        }
    }
}
