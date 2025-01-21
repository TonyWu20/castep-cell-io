use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default,
)]
/// This keyword controls the representation of the nonlocal part of the pseudopotential.
/// Available options are:
/// - RECIPROCAL - reciprocal space nonlocal pseudopotentials.
/// - REAL - real space nonlocal pseudopotentials.
/// # Default
/// The default is the value of `RECIPROCAL`.
/// # Example
/// `PSPOT_NONLOCAL_TYPE : REAL`
pub enum PSPotNonlocalType {
    #[default]
    Reciprocal,
    Real,
}

impl Display for PSPotNonlocalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_uppercase())
    }
}

impl KeywordDisplay for PSPotNonlocalType {
    fn field(&self) -> String {
        "PSPOT_NONLOCAL_TYPE".to_string()
    }
}
