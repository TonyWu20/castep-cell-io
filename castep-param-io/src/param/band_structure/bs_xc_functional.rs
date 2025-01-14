use std::fmt::Display;

use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

use crate::param::XCFunctional;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, KeywordDisplay,
)]
#[keyword_display(direct_display = false, field = "BS_XC_FUNCTIONAL")]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// This keyword controls which functional is used to determine the
/// exchange-correlation potential during a band structure calculation.
///
/// This option allows you to apply screened and exact exchange functionals
/// non self-consistently to obtain more accurate band gaps than with LDA or GGA functionals.
///
/// # Default
/// The default value is derived from the value for XC_FUNCTIONAL.
/// # Example
/// `BS_XC_FUNCTIONAL : PW91`
pub enum BSXcFunctional {
    LDA,
    PW91,
    PBE,
    RPBE,
    WC,
    PBESOL,
    HF,
    HF_LDA,
    SHF,
    SHF_LDA,
    PBE0,
    B3LYP,
    HSE03,
    HSE06,
    RSCAN,
}

impl Display for BSXcFunctional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BSXcFunctional::SHF_LDA => f.write_str("SHF-LDA"),
            BSXcFunctional::HF_LDA => f.write_str("HF-LDA"),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl BSXcFunctional {
    fn default_value(xc_functional: XCFunctional) -> Self {
        match xc_functional {
            XCFunctional::LDA => Self::LDA,
            XCFunctional::PW91 => Self::PW91,
            XCFunctional::PBE => Self::PBE,
            XCFunctional::RPBE => Self::RPBE,
            XCFunctional::WC => Self::WC,
            XCFunctional::PBESOL => Self::PBESOL,
            XCFunctional::BLYP => Self::B3LYP,
            XCFunctional::HF => Self::HF,
            XCFunctional::HF_LDA => Self::HF_LDA,
            XCFunctional::sX => Self::SHF,
            XCFunctional::sX_LDA => Self::SHF_LDA,
            XCFunctional::PBE0 => Self::PBE0,
            XCFunctional::B3LYP => Self::B3LYP,
            XCFunctional::HSE03 => Self::HSE03,
            XCFunctional::HSE06 => Self::HSE06,
            XCFunctional::RSCAN => Self::RSCAN,
        }
    }
}
