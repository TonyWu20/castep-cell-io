use std::fmt::Display;

use super::{OptionDisplay, ParamSectionDisplay, XCFunctional};

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum BandStructureParam {
    BS_EIGENVALUE_TOL(f64),
    BS_MAX_CG_STEPS(u32),
    BS_MAX_ITER(u32),
    BS_NBANDS(u32),
    BS_NEXTRA_BANDS(u32),
    BS_PERC_EXTRA_BANDS(f64),
    BS_RE_EST_K_SCRN(bool),
    BS_XC_FUNCTIONAL(XCFunctional),
    BS_WRITE_EIGENVALUES(bool),
}

#[derive(Debug, Clone)]
pub struct BandStructureParamSection {
    params: Vec<BandStructureParam>,
}

impl BandStructureParamSection {
    pub fn new(params: Vec<BandStructureParam>) -> Self {
        Self { params }
    }

    pub fn params(&self) -> &[BandStructureParam] {
        &self.params
    }
}

impl Default for BandStructureParamSection {
    fn default() -> Self {
        Self::new(vec![
            BandStructureParam::BS_NEXTRA_BANDS(72),
            BandStructureParam::BS_XC_FUNCTIONAL(XCFunctional::default()),
            BandStructureParam::BS_EIGENVALUE_TOL(1e-5),
            BandStructureParam::BS_WRITE_EIGENVALUES(true),
        ])
    }
}

impl OptionDisplay for BandStructureParam {
    fn tag(&self) -> String {
        match self {
            BandStructureParam::BS_EIGENVALUE_TOL(_) => "bs_eigenvalue_tol".to_string(),
            BandStructureParam::BS_MAX_CG_STEPS(_) => "bs_max_cg_steps".to_string(),
            BandStructureParam::BS_MAX_ITER(_) => "bs_max_iter".to_string(),
            BandStructureParam::BS_NBANDS(_) => "bs_nbands".to_string(),
            BandStructureParam::BS_NEXTRA_BANDS(_) => "bs_nextra_bands".to_string(),
            BandStructureParam::BS_PERC_EXTRA_BANDS(_) => "bs_perc_extra_bands".to_string(),
            BandStructureParam::BS_RE_EST_K_SCRN(_) => "bs_re_est_k_scrn".to_string(),
            BandStructureParam::BS_XC_FUNCTIONAL(_) => "bs_xc_functional".to_string(),
            BandStructureParam::BS_WRITE_EIGENVALUES(_) => "bs_write_eigenvalues".to_string(),
        }
    }

    fn value(&self) -> String {
        match self {
            BandStructureParam::BS_EIGENVALUE_TOL(f) => format! {"{f:24.15e}"},
            BandStructureParam::BS_MAX_CG_STEPS(u) => format!("{u}"),
            BandStructureParam::BS_MAX_ITER(u) => format!("{u}"),
            BandStructureParam::BS_NBANDS(u) => format!("{u}"),
            BandStructureParam::BS_NEXTRA_BANDS(u) => format!("{u}"),
            BandStructureParam::BS_PERC_EXTRA_BANDS(f) => format!("{f}"),
            BandStructureParam::BS_RE_EST_K_SCRN(b) => format!("{b}"),
            BandStructureParam::BS_XC_FUNCTIONAL(xc) => xc.value(),
            BandStructureParam::BS_WRITE_EIGENVALUES(b) => format!("{b}"),
        }
    }
}

impl Display for BandStructureParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.output())
    }
}

impl ParamSectionDisplay for BandStructureParamSection {
    fn options(&self) -> &[impl Display] {
        self.params()
    }
}

impl Display for BandStructureParamSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.section_content())
    }
}

#[cfg(test)]
mod test {
    use crate::cell_document::params::options::band_structure_params::BandStructureParamSection;

    #[test]
    fn test_bs_pm() {
        println!("{}", BandStructureParamSection::default())
    }
}
