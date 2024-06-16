use std::fmt::Display;

use super::OptionDisplay;

#[derive(Debug, Default, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum XCFunctional {
    LDA,
    PW91,
    #[default]
    PBE,
    RPBE,
    WC,
    PBESOL,
    HF,
    HFLDA, // HF-LDA
    SX,    // sX
    SXLDA, // sX-LDA
    PBEO,
    B3LYP,
    HSE03,
    HSE06,
}

impl Display for XCFunctional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.output())
    }
}

impl OptionDisplay for XCFunctional {
    fn tag(&self) -> String {
        "xc_functional".to_string()
    }
    fn value(&self) -> String {
        match self {
            XCFunctional::LDA => "LDA".to_string(),
            XCFunctional::PW91 => "PW91".to_string(),
            XCFunctional::PBE => "PBE".to_string(),
            XCFunctional::RPBE => "RPBE".to_string(),
            XCFunctional::WC => "WC".to_string(),
            XCFunctional::PBESOL => "PBESOL".to_string(),
            XCFunctional::HF => "HF".to_string(),
            XCFunctional::HFLDA => "HF-LDA".to_string(),
            XCFunctional::SX => "sX".to_string(),
            XCFunctional::SXLDA => "sX-LDA".to_string(),
            XCFunctional::PBEO => "PBEO".to_string(),
            XCFunctional::B3LYP => "B3LYP".to_string(),
            XCFunctional::HSE03 => "HSE03".to_string(),
            XCFunctional::HSE06 => "HSE06".to_string(),
        }
    }
}
