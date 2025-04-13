use crate::parser::{data_type::PosInteger, Rule};
use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

/// This keyword controls the number of points used to estimate the BASIS_DE_DLOGE in automatic calculation of the finite basis set correction. Points are chosen such that the CUT_OFF_ENERGY corresponds to the highest energy in the set of FINITE_BASIS_NPOINTS cutoff energies.
/// # Note
/// This value is only used if FINITE_BASIS_CORR : 2.
/// # Default
/// 3
/// # Example
/// `FINITE_BASIS_NPOINTS : 5`
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
    KeywordDisplay,
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(field="FINITE_BASIS_NPOINTS", from=u64, value=u64, default_value=3)]
#[pest_ast(rule(Rule::finite_basis_npoints))]
#[pest_rule(rule=Rule::finite_basis_npoints,keyword="FINITE_BASIS_NPOINTS")]
pub struct FiniteBasisNPoints(
    #[pest_ast(inner(
        rule(Rule::pos_integer),
        with(PosInteger::new),
        with(u64::try_from),
        with(Result::unwrap),
    ))]
    u64,
);
