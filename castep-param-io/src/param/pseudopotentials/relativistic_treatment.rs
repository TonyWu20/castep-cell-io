use std::fmt::Display;

use crate::parser::Rule;
use castep_param_derive::BuildFromPairs;
use from_pest::FromPest;
use pest::{Parser, Span};
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

/// Selects the method used to treat relativistic effects. This functionality is implemented for on-the-fly generated pseudopotentials, so this keyword has no effect when pseudopotentials are read from a file.
/// Available options are:
/// - SCHROEDINGER - this option produces completely non-relativistic pseudopotentials.
/// - ZORA - scalar relativistic treatment, which is equivalent to the zeroth-order expansion of the Koelling-Harmon equation.
/// - KOELLING-HARMON - scalar relativistic treatment.
/// - DIRAC - fully relativistic treatment.
/// # Default
/// `KOELLING-HARMON`
/// # Example
/// `RELATIVISTIC_TREATMENT : ZORA`
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Hash,
    Default,
    FromPest,
    BuildFromPairs,
)]
#[pest_ast(rule(Rule::relativistic_treatment))]
#[pest_rule(rule=Rule::relativistic_treatment,keyword="RELATIVISTIC_TREATMENT")]
pub enum RelativisticTreatment {
    #[pest_ast(inner(rule(Rule::relativistic_treatments), with(from_span)))]
    Schroedinger,
    Zora,
    #[default]
    KoellingHarmon,
    Dirac,
}

fn from_span(span: Span<'_>) -> RelativisticTreatment {
    let input = span.as_str();
    match input.to_lowercase().as_str() {
        "schroedinger" => Some(RelativisticTreatment::Schroedinger),
        "zora" => Some(RelativisticTreatment::Zora),
        "koelling-harmon" => Some(RelativisticTreatment::KoellingHarmon),
        "dirac" => Some(RelativisticTreatment::Dirac),
        _ => None,
    }
    .expect("Always correct")
}

impl Display for RelativisticTreatment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelativisticTreatment::Schroedinger => f.write_str("schroedinger"),
            RelativisticTreatment::Zora => f.write_str("zora"),
            RelativisticTreatment::KoellingHarmon => f.write_str("koelling-harmon"),
            RelativisticTreatment::Dirac => f.write_str("dirac"),
        }
    }
}

impl KeywordDisplay for RelativisticTreatment {
    fn field(&self) -> String {
        "RELATIVISTIC_TREATMENT".to_string()
    }
}
