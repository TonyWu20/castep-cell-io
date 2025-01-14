use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
#[allow(clippy::upper_case_acronyms)]
pub enum XCFunctional {
    #[default]
    LDA, // Local Density Approximation
    PW91,   // Perdew Wang '91 GGA
    PBE,    // Perdew Burke Ernzerhof
    RPBE,   // Revised Perdew Burke Ernzerhof
    WC,     // Wu-Cohen
    PBESOL, // PBEsol, PBE functional for solids
    BLYP,   // Becke Lee Young Parr
    HF,     // exact exchange, no correlation
    #[allow(non_camel_case_types)]
    HF_LDA, // exact exchange, LDA correlation
    #[allow(non_camel_case_types)]
    sX, // screened exchange, no correlation
    #[allow(non_camel_case_types)]
    sX_LDA, // screened exchange, LDA correlation
    PBE0,   // PBE0 hybrid functional
    B3LYP,  // B3LYP hybrid functional
    HSE03,  // HSE03 hybrid functional
    HSE06,  // HSE06 hybrid functional
    RSCAN,  // regularized SCAN meta-GGA functional
}

impl Display for XCFunctional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl KeywordDisplay for XCFunctional {
    fn field(&self) -> String {
        "XC_FUNCTIONAL".to_string()
    }
}
