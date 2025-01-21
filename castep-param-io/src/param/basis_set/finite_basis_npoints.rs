use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

/// This keyword controls the number of points used to estimate the BASIS_DE_DLOGE in automatic calculation of the finite basis set correction. Points are chosen such that the CUT_OFF_ENERGY corresponds to the highest energy in the set of FINITE_BASIS_NPOINTS cutoff energies.
/// # Note
/// This value is only used if FINITE_BASIS_CORR : 2.
/// # Default
/// 3
/// # Example
/// `FINITE_BASIS_NPOINTS : 5`
#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, KeywordDisplay,
)]
#[keyword_display(field="FINITE_BASIS_NPOINTS", from=u32, value=u32, default_value=3)]
pub struct FiniteBasisNPoints(u32);
