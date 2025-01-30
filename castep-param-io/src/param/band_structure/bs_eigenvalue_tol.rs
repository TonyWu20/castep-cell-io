use crate::parser::Rule;
use castep_param_derive::{BuildFromPairs, KeywordDisplayStruct};
use derive_builder::Builder;
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::{param::EnergyUnit, parser::data_type::Real};

/// This keyword controls the tolerance for accepting convergence of a
/// single eigenvalue or band during a band structure calculation.
/// # Note
/// The difference between maximum and minimum eigenvalue for every band
/// over ELEC_CONVERGENCE_WIN iterations must be less than this value.
/// # Default
/// 1x10-6 eV
/// # Example
/// `BS_EIGENVALUE_TOL = 1.0e-5 Ha`
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
    Builder,
    KeywordDisplayStruct,
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(
    field = "BS_EIGENVALUE_TOL",
    from = f64,
    default_value = 1e-6,
    display_format = "{:20.15e} {}",
)]
#[pest_ast(rule(Rule::bs_eigenvalue_tol))]
#[pest_rule(rule=Rule::bs_eigenvalue_tol,keyword="BS_EIGENVALUE_TOL")]
pub struct BSEigenvalueTol {
    #[pest_ast(inner(rule(Rule::real), with(Real::from_span), with(f64::from)))]
    pub tol: f64,
    #[keyword_display(is_option = true)]
    #[pest_ast(inner(rule(Rule::bs_eigenvalue_tol_unit), with(EnergyUnit::from_span)))]
    pub unit: Option<EnergyUnit>,
}

#[cfg(test)]
mod test {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::parser::{ParamParser, Rule};

    use super::BSEigenvalueTol;

    #[test]
    fn bs_eigenvalue_tol_parse() {
        let input = "bs_eigenvalue_tol : 1e-5";
        let mut parse = ParamParser::parse(Rule::bs_eigenvalue_tol, input).unwrap();
        let tol = BSEigenvalueTol::from_pest(&mut parse).unwrap();
        dbg!(tol);
        let input_with_unit = "bs_eigenvalue_tol : 1e-5 Ha";
        let mut parse = ParamParser::parse(Rule::bs_eigenvalue_tol, input_with_unit).unwrap();
        let tol = BSEigenvalueTol::from_pest(&mut parse).unwrap();
        dbg!(tol);
    }
}
