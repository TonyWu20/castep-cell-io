use std::fmt::Display;

use castep_param_derive::KeywordDisplay;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(Debug, Clone, Serialize, Deserialize, Builder, PartialEq, PartialOrd)]
#[builder(setter(into, strip_option), default)]
pub struct XCDefinition {
    functional_weight: Vec<FunctionalWeight>,
    /// The NLXC_SCREENING_LENGTH parameter can be used to set the screening length value in Bohr-1. Default value is 1.437 Å-1 = 0.76 Bohr-1.
    nlxc_screening_length: Option<f64>,
    /// NLXC_SCREENING_FUNCTION values can be set to either THOMAS-FERMI (default) or ERRORFUNCTION.
    nlxc_screening_function: Option<NLXC_ScreeningFunction>,
    /// NLXC_DIVERGENCE_CORR can be set to either ON or OFF. Divergence correction is ON by default when using unscreened Hartree-Fock (HF) exchange, including such functionals as B3LYP or PBE0; and OFF by default when using screened HF exchange (functionals like SX-LDA, HSE03, HSE06).
    nlxc_divergence_corr: Option<NLXC_DivergenceCorr>,
    nlxc_ppd_int: Option<NLXC_PPD_Int>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum FunctionalWeight {
    LDA(f64),      // Local Density Approximation
    LDA_X(f64),    // Local Density Approximation, exchange only
    LDA_C(f64),    // Local Density Approximation, correlation only
    PW91(f64),     // Perdew Wang '91 GGA
    PBE(f64),      // Perdew Burke Ernzerhof GGA functional
    PBE_X(f64),    // Perdew Burke Ernzerhof GGA functional, exchange only
    PBE_C(f64),    // Perdew Burke Ernzerhof GGA functional, correlation only
    RPBE(f64),     // Revised Perdew Burke Ernzerhof GGA functional
    WC(f64),       // Wu-Cohen GGA functional
    PBESOL(f64),   // PBEsol, PBE functional for solids
    PBESOL_X(f64), // PBEsol, PBE functional for solids, exchange only
    PBESOL_C(f64), // PBEsol, PBE functional for solids, correlation only
    B88_X(f64),    // Becke (1988) GGA functional , exchange only
    LYP_C(f64),    // Lee-Yang-Parr (1988) GGA functional, correlation only
    HF(f64),       // exact exchange, no correlation
    HF_LDA(f64),   // exact exchange, LDA correlation
    SX(f64),       // screened exchange, no correlation
    SX_LDA(f64),   // screened exchange, LDA correlation
    PBE0(f64),     // PBE0 hybrid functional
    B3LYP(f64),    // B3LYP hybrid functional
    HSE03(f64),    // HSE03 hybrid functional
    HSE06(f64),    // HSE06 hybrid functional
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deserialize,
    Serialize,
    Default,
    KeywordDisplay,
)]
#[allow(non_camel_case_types)]
#[keyword_display(field = "NLXC_SCREENING_FUNCTION", direct_display = false)]
pub enum NLXC_ScreeningFunction {
    #[default]
    ThomasFermi,
    ErrorFunction,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, KeywordDisplay,
)]
#[allow(non_camel_case_types)]
#[keyword_display(field = "NLXC_PPD_INT")]
pub enum NLXC_PPD_Int {
    On,
    Off,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, KeywordDisplay,
)]
#[allow(non_camel_case_types)]
#[keyword_display(field = "NLXC_DIVERGENCE_CORR")]
pub enum NLXC_DivergenceCorr {
    On,
    Off,
}

impl Display for FunctionalWeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionalWeight::LDA(w)
            | FunctionalWeight::LDA_X(w)
            | FunctionalWeight::LDA_C(w)
            | FunctionalWeight::PW91(w)
            | FunctionalWeight::PBE(w)
            | FunctionalWeight::PBE_X(w)
            | FunctionalWeight::PBE_C(w)
            | FunctionalWeight::RPBE(w)
            | FunctionalWeight::WC(w)
            | FunctionalWeight::PBESOL(w)
            | FunctionalWeight::PBESOL_X(w)
            | FunctionalWeight::PBESOL_C(w)
            | FunctionalWeight::B88_X(w)
            | FunctionalWeight::LYP_C(w)
            | FunctionalWeight::HF(w)
            | FunctionalWeight::HF_LDA(w)
            | FunctionalWeight::SX(w)
            | FunctionalWeight::SX_LDA(w)
            | FunctionalWeight::PBE0(w)
            | FunctionalWeight::B3LYP(w)
            | FunctionalWeight::HSE03(w)
            | FunctionalWeight::HSE06(w) => write!(f, "{w}"),
        }
    }
}

impl KeywordDisplay for FunctionalWeight {
    fn field(&self) -> String {
        match self {
            FunctionalWeight::LDA(_) => "LDA".to_string(),
            FunctionalWeight::LDA_X(_) => "LDA-X".to_string(),
            FunctionalWeight::LDA_C(_) => "LDA-C".to_string(),
            FunctionalWeight::PW91(_) => "PW91".to_string(),
            FunctionalWeight::PBE(_) => "PBE".to_string(),
            FunctionalWeight::PBE_X(_) => "PBE_X".to_string(),
            FunctionalWeight::PBE_C(_) => "PBE_C".to_string(),
            FunctionalWeight::RPBE(_) => "RPBE".to_string(),
            FunctionalWeight::WC(_) => "WC".to_string(),
            FunctionalWeight::PBESOL(_) => "PBESOL".to_string(),
            FunctionalWeight::PBESOL_X(_) => "PBESOL_X".to_string(),
            FunctionalWeight::PBESOL_C(_) => "PBESOL_C".to_string(),
            FunctionalWeight::B88_X(_) => "B88_X".to_string(),
            FunctionalWeight::LYP_C(_) => "LYP_C".to_string(),
            FunctionalWeight::HF(_) => "HF".to_string(),
            FunctionalWeight::HF_LDA(_) => "HF_LDA".to_string(),
            FunctionalWeight::SX(_) => "SX".to_string(),
            FunctionalWeight::SX_LDA(_) => "SX-LDA".to_string(),
            FunctionalWeight::PBE0(_) => "PBE0".to_string(),
            FunctionalWeight::B3LYP(_) => "B3LYP".to_string(),
            FunctionalWeight::HSE03(_) => "HSE03".to_string(),
            FunctionalWeight::HSE06(_) => "HSE06".to_string(),
        }
    }
}

impl Display for NLXC_ScreeningFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NLXC_ScreeningFunction::ThomasFermi => f.write_str("THOMAS-FERMI"),
            NLXC_ScreeningFunction::ErrorFunction => f.write_str("ERRORFUNCTION"),
        }
    }
}

impl Default for XCDefinition {
    fn default() -> Self {
        Self {
            functional_weight: vec![FunctionalWeight::HF(1.0)],
            nlxc_screening_length: None,
            nlxc_screening_function: None,
            nlxc_divergence_corr: None,
            nlxc_ppd_int: None,
        }
    }
}

impl Display for XCDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = [
            Some("%BLOCK xc_definition".to_string()),
            Some(
                self.functional_weight
                    .iter()
                    .map(|f| f.output())
                    .collect::<Vec<String>>()
                    .join("\n"),
            ),
            self.nlxc_screening_length
                .map(|v| format!("NLXC_SCREENING_LENGTH {v}")),
            self.nlxc_screening_function.map(|v| v.output()),
            self.nlxc_ppd_int.map(|v| v.output()),
            self.nlxc_divergence_corr.map(|v| v.output()),
            Some("%ENDBLOCK xc_definition".to_string()),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<String>>()
        .join("\n");
        write!(f, "{}", output)
    }
}
