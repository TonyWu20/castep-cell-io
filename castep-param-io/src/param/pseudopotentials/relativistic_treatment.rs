use std::fmt::Display;

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
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, Default,
)]
pub enum RelativisticTreatment {
    Schroedinger,
    Zora,
    #[default]
    KoellingHarmon,
    Dirac,
}

impl Display for RelativisticTreatment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelativisticTreatment::Schroedinger => f.write_str("sCHROEDINGER"),
            RelativisticTreatment::Zora => f.write_str("ZORA"),
            RelativisticTreatment::KoellingHarmon => f.write_str("KOELLING-HARMON"),
            RelativisticTreatment::Dirac => f.write_str("DIRAC"),
        }
    }
}

impl KeywordDisplay for RelativisticTreatment {
    fn field(&self) -> String {
        "RELATIVISTIC_TREATMENT".to_string()
    }
}
