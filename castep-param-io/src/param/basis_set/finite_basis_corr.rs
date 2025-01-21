use std::fmt::Display;

use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

/// This keyword determines whether or not to apply a finite basis set correction to energy and stress when cell parameters change.
/// Available options are:
/// - 0 - no correction.
/// - 1 - manual correction using specified BASIS_DE_DLOGE.
/// - 2 - automatic correction using FINITE_BASIS_NPOINTS and FINITE_BASIS_SPACING.
/// # Note
/// - If FINITE_BASIS_CORR : 1, a value for BASIS_DE_DLOGE must be supplied.
/// - You should turn off finite basis set correction when using a fixed size basis (see `FixedNPW`).
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
    Default,
    KeywordDisplay,
)]
#[keyword_display(direct_display = false, field = "FINITE_BASIS_CORR")]
pub enum FiniteBasisCorr {
    /// 0
    NoCorrection,
    /// 1
    Manual,
    #[default]
    /// 2
    Auto,
}

impl Display for FiniteBasisCorr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FiniteBasisCorr::NoCorrection => f.write_str("0"),
            FiniteBasisCorr::Manual => f.write_str("1"),
            FiniteBasisCorr::Auto => f.write_str("2"),
        }
    }
}
