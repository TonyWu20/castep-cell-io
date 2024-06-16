use std::fmt::Display;

use super::{OptionDisplay, SectionDisplay};

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum PopulationParam {
    PDOS_CALCULATE_WEIGHTS(bool),
    POPN_BOND_CUTOFF(f64),
    POPN_CALCULATE(bool),
    POPN_WRITE(PopnWrite),
}

#[derive(Debug, Clone)]
pub struct PopulationAnalysisSection {
    params: Vec<PopulationParam>,
}

impl PopulationAnalysisSection {
    pub fn new(params: Vec<PopulationParam>) -> Self {
        Self { params }
    }

    pub fn params(&self) -> &[PopulationParam] {
        &self.params
    }
}

impl Default for PopulationAnalysisSection {
    fn default() -> Self {
        Self::new(vec![
            PopulationParam::PDOS_CALCULATE_WEIGHTS(true),
            PopulationParam::POPN_BOND_CUTOFF(3_f64),
            PopulationParam::POPN_CALCULATE(true),
        ])
    }
}

impl OptionDisplay for PopulationParam {
    fn tag(&self) -> String {
        match self {
            PopulationParam::PDOS_CALCULATE_WEIGHTS(_) => "pdos_calculate_weights".to_string(),
            PopulationParam::POPN_BOND_CUTOFF(_) => "popn_bond_cutoff".to_string(),
            PopulationParam::POPN_CALCULATE(_) => "popn_calculate".to_string(),
            PopulationParam::POPN_WRITE(_) => "popn_write".to_string(),
        }
    }

    fn value(&self) -> String {
        match self {
            PopulationParam::PDOS_CALCULATE_WEIGHTS(b) => format!("{b}"),
            PopulationParam::POPN_BOND_CUTOFF(f) => format!("{f:24.15}"),
            PopulationParam::POPN_CALCULATE(b) => format!("{b}"),
            PopulationParam::POPN_WRITE(p) => format!("{p}"),
        }
    }
}

impl Display for PopulationParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.output())
    }
}

impl SectionDisplay for PopulationAnalysisSection {
    fn options(&self) -> &[impl Display] {
        self.params()
    }
}

impl Display for PopulationAnalysisSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.section_content())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum PopnWrite {
    None,
    Minimal,
    Summary,
    #[default]
    Enhanced,
    Verbose,
}

impl Display for PopnWrite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PopnWrite::None => f.write_str("None"),
            PopnWrite::Minimal => f.write_str("Minimal"),
            PopnWrite::Summary => f.write_str("Summary"),
            PopnWrite::Enhanced => f.write_str("Enhanced"),
            PopnWrite::Verbose => f.write_str("Verbose"),
        }
    }
}
