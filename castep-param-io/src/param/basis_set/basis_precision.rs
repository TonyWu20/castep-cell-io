use crate::parser::Rule;
use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::{Parser, Span};
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

/// This keywords specifies the precision of the basis set by choosing the level of convergence of atomic energies with respect to the plane wave cutoff energy for the pseudopotentials used in the calculation.
/// Available options are:
/// - COARSE - convergence of about 1 eV/atom
/// - MEDIUM - convergence of about 0.3 eV/atom
/// - FINE - convergence of about 0.1 eV/atom
/// - PRECISE - 1.2 × FINE cutoff energy
/// - EXTREME - 1.6 × FINE cutoff energy, convergence of about 0.01 eV/atom
///
/// If the `BASIS_PRECISION` is defined, the `CUT_OFF_ENERGY` will be equal to the highest of the cutoff energies associated with the chosen level of accuracy, for the pseudopotentials used in the calculation.
/// # Note
/// It is not possible to specify both the `BASIS_PRECISION` and the `CUT_OFF_ENERGY` in a single file.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    Serialize,
    Deserialize,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    KeywordDisplay,
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(field = "BASIS_PRECISION")]
#[pest_ast(rule(Rule::basis_precision))]
#[pest_rule(rule=Rule::basis_precision,keyword="BASIS_PRECISION")]
pub enum BasisPrecision {
    #[pest_ast(inner(rule(Rule::basis_precisions), with(from_span)))]
    Coarse,
    Medium,
    #[default]
    Fine,
    Precise,
    Extreme,
}

fn from_span(span: Span<'_>) -> BasisPrecision {
    let input = span.as_str();
    match input.to_lowercase().as_str() {
        "coarse" => Some(BasisPrecision::Coarse),
        "medium" => Some(BasisPrecision::Medium),
        "fine" => Some(BasisPrecision::Fine),
        "precise" => Some(BasisPrecision::Precise),
        "extreme" => Some(BasisPrecision::Extreme),
        _ => None,
    }
    .expect("Always correct")
}
