use std::fmt::Display;

use crate::cell_document::params::options::OptionDisplay;

#[derive(Debug, Default, Copy, Clone)]
pub enum FiniteBasisCorr {
    #[default]
    No,
    Manual,
    Auto,
}

impl Display for FiniteBasisCorr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.output())
    }
}

impl OptionDisplay for FiniteBasisCorr {
    fn tag(&self) -> String {
        "finite_basis_corr".to_string()
    }

    fn value(&self) -> String {
        match self {
            Self::No => "0".to_string(),
            Self::Manual => "1".to_string(),
            Self::Auto => "2".to_string(),
        }
    }
}
