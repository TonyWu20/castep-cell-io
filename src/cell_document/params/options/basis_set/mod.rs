use std::{
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use finite_basis_corr::FiniteBasisCorr;
use thiserror::Error;

use super::{OptionDisplay, ParamSectionDisplay};

mod finite_basis_corr;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum EnergyCutoff {
    Coarse,
    Medium,
    Fine,
    #[default]
    Ultrafine,
}

#[derive(Error, Debug)]
pub enum EnergyCutoffError {
    #[error("open potential file failed. {0}")]
    OpenPotentialFile(#[from] io::Error),
}

impl EnergyCutoff {
    pub fn get_cutoff_energy_from_pot<P: AsRef<Path>>(
        &self,
        potential_file: P,
    ) -> Result<f64, EnergyCutoffError> {
        let file = File::open(potential_file.as_ref())?;
        let reader = BufReader::new(file);
        let keyword = match self {
            EnergyCutoff::Coarse => "COARSE",
            EnergyCutoff::Medium => "MEDIUM",
            EnergyCutoff::Fine | EnergyCutoff::Ultrafine => "FINE",
        };
        let cutoff_energy = reader
            .lines()
            .find(|line| line.as_ref().unwrap().contains(keyword))
            .map(|line| {
                // This pattern does not handle `otfg` file
                // Support in future
                let num_str = line.as_ref().unwrap().split_whitespace().next().unwrap();
                num_str.parse::<u32>().expect("Can't parse into `u32`")
            })
            .expect("Failed to parse fine energy from pseudopotential file.");
        let round_bigger_tenth = |num: u32| -> f64 {
            match num % 10 {
                0 => num as f64,
                _ => ((num / 10 + 1) * 10) as f64,
            }
        };
        let multipler = match self {
            EnergyCutoff::Ultrafine => 1.1,
            _ => 1.0,
        };
        Ok(round_bigger_tenth(
            (cutoff_energy as f64 * multipler) as u32,
        ))
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum BasisSetParam {
    // BASIS_DE_DLOGE,
    // BASIS_PRECISION,
    CUT_OFF_ENERGY(f64),
    // FINE_GMAX,
    FINE_GRID_SCALE(f64),
    FINITE_BASIS_CORR(FiniteBasisCorr),
    // FINITE_BASIS_NPOINTS,
    // FINITE_BASIS_SPACING,
    FIXED_NPW(bool),
    GRID_SCALE(f64),
}

#[derive(Debug, Clone)]
pub struct BasisSetParamSection {
    params: Vec<BasisSetParam>,
}

impl BasisSetParamSection {
    pub fn new(params: Vec<BasisSetParam>) -> Self {
        Self { params }
    }
    pub fn with_cutoff_energy(cutoff_energy: f64) -> Self {
        Self::new(vec![
            BasisSetParam::CUT_OFF_ENERGY(cutoff_energy),
            BasisSetParam::GRID_SCALE(1.5),
            BasisSetParam::FINE_GRID_SCALE(1.5),
            BasisSetParam::FINITE_BASIS_CORR(FiniteBasisCorr::default()),
            BasisSetParam::FIXED_NPW(false),
        ])
    }
}

impl Default for BasisSetParamSection {
    fn default() -> Self {
        Self::new(vec![
            BasisSetParam::CUT_OFF_ENERGY(0.0),
            BasisSetParam::GRID_SCALE(1.5),
            BasisSetParam::FINE_GRID_SCALE(1.5),
            BasisSetParam::FINITE_BASIS_CORR(FiniteBasisCorr::default()),
            BasisSetParam::FIXED_NPW(false),
        ])
    }
}

impl OptionDisplay for BasisSetParam {
    fn tag(&self) -> String {
        match self {
            BasisSetParam::CUT_OFF_ENERGY(_) => "cut_off_energy".to_string(),
            BasisSetParam::FINE_GRID_SCALE(_) => "fine_grid_scale".to_string(),
            BasisSetParam::FINITE_BASIS_CORR(_) => "finite_basis_corr".to_string(),
            BasisSetParam::FIXED_NPW(_) => "fixed_npw".to_string(),
            BasisSetParam::GRID_SCALE(_) => "grid_scale".to_string(),
        }
    }

    fn value(&self) -> String {
        match self {
            BasisSetParam::CUT_OFF_ENERGY(f) => format!("{f:20.15}"),
            BasisSetParam::GRID_SCALE(f) => format!("{f:20.15}"),
            BasisSetParam::FINE_GRID_SCALE(f) => format!("{f:20.15}"),
            BasisSetParam::FINITE_BASIS_CORR(fbc) => fbc.value(),
            BasisSetParam::FIXED_NPW(b) => format!("{b}"),
        }
    }
}

impl Display for BasisSetParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.output())
    }
}

impl ParamSectionDisplay for BasisSetParamSection {
    fn options(&self) -> &[impl Display] {
        &self.params
    }
}

impl Display for BasisSetParamSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.section_content())
    }
}

#[cfg(test)]
mod test {
    use super::BasisSetParamSection;

    #[test]
    fn test_basis_set_pm() {
        let pm = BasisSetParamSection::with_cutoff_energy(380.0);
        println!("{pm}");
    }
}
