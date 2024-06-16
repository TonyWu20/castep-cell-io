use std::fmt::Display;

use super::{OptionDisplay, SectionDisplay};

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum GeomOptParam {
    GEOM_CONVERGENCE_WIN(u32),
    GEOM_DISP_TOL(f64),
    GEOM_ENERGY_TOL(f64),
    GEOM_FORCE_TOL(f64),
    GEOM_FREQUENCY_EST(f64),
    GEOM_MAX_ITER(u32),
    GEOM_METHOD(GeomMethod),
    GEOM_MODULUS_EST(f64),
    GEOM_PRECONDITIONER(GeomPreConditioner),
    GEOM_SPIN_FIX(u32),
    GEOM_STRESS_TOL(f64),
}

#[derive(Debug, Clone)]
pub struct GeomOptParamSection {
    params: Vec<GeomOptParam>,
}

impl GeomOptParamSection {
    pub fn new(params: Vec<GeomOptParam>) -> Self {
        Self { params }
    }

    pub fn params(&self) -> &[GeomOptParam] {
        &self.params
    }
}

impl Default for GeomOptParamSection {
    fn default() -> Self {
        Self::new(vec![
            GeomOptParam::GEOM_ENERGY_TOL(5e-5),
            GeomOptParam::GEOM_FORCE_TOL(0.1),
            GeomOptParam::GEOM_STRESS_TOL(0.2),
            GeomOptParam::GEOM_DISP_TOL(0.005),
            GeomOptParam::GEOM_MAX_ITER(6000),
            GeomOptParam::GEOM_METHOD(GeomMethod::default()),
        ])
    }
}

impl OptionDisplay for GeomOptParam {
    fn tag(&self) -> String {
        match self {
            GeomOptParam::GEOM_CONVERGENCE_WIN(_) => "geom_convergence_win".to_string(),
            GeomOptParam::GEOM_DISP_TOL(_) => "geom_disp_tol".to_string(),
            GeomOptParam::GEOM_ENERGY_TOL(_) => "geom_energy_tol".to_string(),
            GeomOptParam::GEOM_FORCE_TOL(_) => "geom_force_tol".to_string(),
            GeomOptParam::GEOM_FREQUENCY_EST(_) => "geom_frequency_est".to_string(),
            GeomOptParam::GEOM_MAX_ITER(_) => "geom_max_iter".to_string(),
            GeomOptParam::GEOM_METHOD(_) => "geom_method".to_string(),
            GeomOptParam::GEOM_MODULUS_EST(_) => "geom_modulus_est".to_string(),
            GeomOptParam::GEOM_PRECONDITIONER(_) => "geom_preconditioner".to_string(),
            GeomOptParam::GEOM_SPIN_FIX(_) => "geom_spin_fix".to_string(),
            GeomOptParam::GEOM_STRESS_TOL(_) => "geom_stress_tol".to_string(),
        }
    }

    fn value(&self) -> String {
        match self {
            GeomOptParam::GEOM_CONVERGENCE_WIN(u) => format!("{u}"),
            GeomOptParam::GEOM_DISP_TOL(f) => format!("{f:24.15}"),
            GeomOptParam::GEOM_ENERGY_TOL(f) => format!("{f:24.15e}"),
            GeomOptParam::GEOM_FORCE_TOL(f) => format!("{f:24.15}"),
            GeomOptParam::GEOM_FREQUENCY_EST(f) => format!("{f:24.15}"),
            GeomOptParam::GEOM_MAX_ITER(u) => format!("{u}"),
            GeomOptParam::GEOM_METHOD(gm) => format!("{gm}"),
            GeomOptParam::GEOM_MODULUS_EST(f) => format!("{f:24.15}"),
            GeomOptParam::GEOM_PRECONDITIONER(gp) => format!("{gp}"),
            GeomOptParam::GEOM_SPIN_FIX(u) => format!("{u}"),
            GeomOptParam::GEOM_STRESS_TOL(f) => format!("{f:24.15}"),
        }
    }
}

impl Display for GeomOptParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.output())
    }
}

impl SectionDisplay for GeomOptParamSection {
    fn options(&self) -> &[impl Display] {
        self.params()
    }
}

impl Display for GeomOptParamSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.section_content())
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum GeomMethod {
    #[default]
    BFGS,
    LBFGS,
    Delocalized,
    DampedMD,
    TPSD,
}

impl Display for GeomMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeomMethod::BFGS => f.write_str("BFGS"),
            GeomMethod::LBFGS => f.write_str("LBFGS"),
            GeomMethod::Delocalized => f.write_str("Delocalized"),
            GeomMethod::DampedMD => f.write_str("DampedMD"),
            GeomMethod::TPSD => f.write_str("TPSD"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum GeomPreConditioner {
    ID,
    EXP,
    FF,
}

impl Display for GeomPreConditioner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeomPreConditioner::ID => f.write_str("ID"),
            GeomPreConditioner::EXP => f.write_str("EXP"),
            GeomPreConditioner::FF => f.write_str("FF"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::GeomOptParamSection;

    #[test]
    fn test_geom_pm() {
        let pm = GeomOptParamSection::default();
        println!("{pm}");
    }
}
