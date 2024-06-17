use std::fmt::Display;

use crate::cell_document::params::options::OptionDisplay;

#[derive(Debug, Default, Clone, Copy)]
pub enum OptStrategy {
    Default,
    #[default]
    Speed,
    Memory,
}

impl Display for OptStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.output())
    }
}

impl OptionDisplay for OptStrategy {
    fn tag(&self) -> String {
        "opt_strategy".to_string()
    }

    fn value(&self) -> String {
        match self {
            Self::Default => "Default".to_string(),
            Self::Speed => "Speed".to_string(),
            Self::Memory => "Memory".to_string(),
        }
    }
}
