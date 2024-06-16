use std::fmt::Display;

use finite_basis_corr::FiniteBasisCorr;

use super::{OptionDisplay, SectionDisplay};

mod finite_basis_corr;

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
    pub fn set_cutoff(&mut self, cutoff_energy: f64) {
        self.params
            .iter_mut()
            .filter(|p| matches!(p, BasisSetParam::CUT_OFF_ENERGY(_)))
            .for_each(|p| *p = BasisSetParam::CUT_OFF_ENERGY(cutoff_energy));
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

impl SectionDisplay for BasisSetParamSection {
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
        let mut pm = BasisSetParamSection::default();
        pm.set_cutoff(380.0);
        println!("{pm}");
    }
}
