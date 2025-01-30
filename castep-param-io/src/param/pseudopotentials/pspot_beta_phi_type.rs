use std::fmt::Display;

use castep_param_derive::BuildFromPairs;
use from_pest::FromPest;
use pest::{Parser, Span};
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;
use crate::parser::Rule;

#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    FromPest,
    BuildFromPairs,
)]
/// This keyword controls the representation of the nonlocal part of the pseudopotential used for calculation of the <β|ϕ> overlaps.
/// Available options are:
/// - RECIPROCAL - reciprocal space nonlocal pseudopotentials
/// - REAL - real space nonlocal pseudopotentials
/// # Note
/// This parameter can only take the value REAL if PSPOT_NONLOCAL_TYPE is also REAL.
/// # Default
/// The default is the value of `PSPOT_NONLOCAL_TYPE`
/// # Example
/// `PSPOT_BETA_PHI_TYPE : REAL`
#[pest_ast(rule(Rule::pspot_beta_phi_type))]
#[pest_rule(rule=Rule::pspot_beta_phi_type,keyword="PSPOT_BETA_PHI_TYPE")]
pub enum PSPotBetaPhiType {
    #[pest_ast(inner(rule(Rule::pspot_beta_phi_types), with(from_span)))]
    Reciprocal,
    Real,
}

fn from_span(span: Span<'_>) -> PSPotBetaPhiType {
    let input = span.as_str();
    match input.to_lowercase().as_str() {
        "reciprocal" => Some(PSPotBetaPhiType::Reciprocal),
        "real" => Some(PSPotBetaPhiType::Real),
        _ => None,
    }
    .expect("Always correct")
}

impl Display for PSPotBetaPhiType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PSPotBetaPhiType::Reciprocal => f.write_str("reciprocal"),
            PSPotBetaPhiType::Real => f.write_str("real"),
        }
    }
}

impl KeywordDisplay for PSPotBetaPhiType {
    fn field(&self) -> String {
        "PSPOT_BETA_PHI_TYPE".to_string()
    }
}
