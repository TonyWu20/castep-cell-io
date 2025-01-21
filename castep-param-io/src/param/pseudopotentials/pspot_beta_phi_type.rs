use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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
pub enum PSPotBetaPhiType {
    Reciprocal,
    Real,
}

impl Display for PSPotBetaPhiType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_uppercase())
    }
}

impl KeywordDisplay for PSPotBetaPhiType {
    fn field(&self) -> String {
        "PSPOT_BETA_PHI_TYPE".to_string()
    }
}
