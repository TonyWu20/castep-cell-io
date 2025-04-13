use std::fmt::Display;

use crate::parser::Rule;
use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::{Parser, Span};
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::param::XCFunctional;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Hash,
    KeywordDisplay,
    FromPest,
    BuildFromPairs,
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
#[pest_ast(rule(Rule::bs_xc_functional))]
#[pest_rule(rule=Rule::bs_xc_functional,keyword="BS_XC_FUNCTIONAL")]
pub enum BSXcFunctional {
    #[pest_ast(inner(rule(Rule::bs_xc_functionals), with(from_span)))]
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

fn from_span(span: Span<'_>) -> BSXcFunctional {
    let input = span.as_str();
    match input.to_lowercase().as_str() {
        "shf-lda" => Some(BSXcFunctional::SHF_LDA),
        "hf-lda" => Some(BSXcFunctional::HF_LDA),
        "lda" => Some(BSXcFunctional::LDA),
        "pw91" => Some(BSXcFunctional::PW91),
        "pbe" => Some(BSXcFunctional::PBE),
        "rpbe" => Some(BSXcFunctional::RPBE),
        "wc" => Some(BSXcFunctional::WC),
        "pbesol" => Some(BSXcFunctional::PBESOL),
        "hf" => Some(BSXcFunctional::HF),
        "shf" => Some(BSXcFunctional::SHF),
        "pbe0" => Some(BSXcFunctional::PBE0),
        "b3lyp" => Some(BSXcFunctional::B3LYP),
        "hse03" => Some(BSXcFunctional::HSE03),
        "hse06" => Some(BSXcFunctional::HSE06),
        "rscan" => Some(BSXcFunctional::RSCAN),
        _ => None,
    }
    .expect("Always correct")
}

impl Display for BSXcFunctional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BSXcFunctional::SHF_LDA => f.write_str("SHF-LDA"),
            BSXcFunctional::HF_LDA => f.write_str("HF-LDA"),
            BSXcFunctional::LDA => f.write_str("lda"),
            BSXcFunctional::PW91 => f.write_str("pw91"),
            BSXcFunctional::PBE => f.write_str("pbe"),
            BSXcFunctional::RPBE => f.write_str("rpbe"),
            BSXcFunctional::WC => f.write_str("wc"),
            BSXcFunctional::PBESOL => f.write_str("pbesol"),
            BSXcFunctional::HF => f.write_str("hf"),
            BSXcFunctional::SHF => f.write_str("shf"),
            BSXcFunctional::PBE0 => f.write_str("pbe0"),
            BSXcFunctional::B3LYP => f.write_str("b3lyp"),
            BSXcFunctional::HSE03 => f.write_str("hse03"),
            BSXcFunctional::HSE06 => f.write_str("hse06"),
            BSXcFunctional::RSCAN => f.write_str("rscan"),
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
