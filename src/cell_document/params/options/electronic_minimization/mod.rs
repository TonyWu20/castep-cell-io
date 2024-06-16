use std::fmt::Display;

use metals_method::{DensityMixing, Edft, MetalsMethod};

use super::{OptionDisplay, SectionDisplay};

mod metals_method;
/// Electronic minimization parameters
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum ElectroMinParam {
    EFERMI_TOL,
    ELEC_CONVERGENCE_WIN,
    ELEC_DUMP_FILE,
    ELEC_EIGENVALUE_TOL,
    ELEC_ENERGY_TOL(f64),
    ELEC_RESTORE_FILE,
    ELECTRONIC_MINIMIZER,
    FIX_OCCUPANCY(bool),
    MAX_CG_STEPS,
    MAX_SCF_CYCLES(u32),
    MAX_SD_STEPS,
    METALS_METHOD(MetalsMethod),
    NUM_DUMP_CYCLES(u32),
    SMEARING_SCHEME,
    SMEARING_WIDTH(f64),
    SPIN_FIX(u32),
}

#[derive(Debug, Clone)]
pub struct ElectroMinParamSection {
    params: Vec<ElectroMinParam>,
}

impl ElectroMinParamSection {
    pub fn new(params: Vec<ElectroMinParam>) -> Self {
        Self { params }
    }
    pub fn with_edft() -> Self {
        let mut to_use_param = Self::default();
        to_use_param
            .params
            .iter_mut()
            .filter(|p| matches!(p, ElectroMinParam::METALS_METHOD(_)))
            .for_each(|p| *p = ElectroMinParam::METALS_METHOD(MetalsMethod::Edft(Edft::default())));
        to_use_param
    }
}

impl Default for ElectroMinParamSection {
    fn default() -> Self {
        let params = vec![
            ElectroMinParam::ELEC_ENERGY_TOL(1e-5),
            ElectroMinParam::FIX_OCCUPANCY(false),
            ElectroMinParam::METALS_METHOD(MetalsMethod::Dm(DensityMixing::default())),
            ElectroMinParam::MAX_SCF_CYCLES(6000),
            ElectroMinParam::SMEARING_WIDTH(0.1_f64),
            ElectroMinParam::SPIN_FIX(6),
        ];
        Self::new(params)
    }
}

impl OptionDisplay for ElectroMinParam {
    fn tag(&self) -> String {
        match self {
            ElectroMinParam::EFERMI_TOL => "efermi_tol".to_string(),
            ElectroMinParam::ELEC_CONVERGENCE_WIN => "elec_convergence_win".to_string(),
            ElectroMinParam::ELEC_DUMP_FILE => "elec_dump_file".to_string(),
            ElectroMinParam::ELEC_EIGENVALUE_TOL => "elec_eigenvalue_tol".to_string(),
            ElectroMinParam::ELEC_ENERGY_TOL(_) => "elec_energy_tol".to_string(),
            ElectroMinParam::ELEC_RESTORE_FILE => "elec_restore_file".to_string(),
            ElectroMinParam::ELECTRONIC_MINIMIZER => "electronic_minimizer".to_string(),
            ElectroMinParam::FIX_OCCUPANCY(_) => "fix_occupancy".to_string(),
            ElectroMinParam::MAX_CG_STEPS => "max_cg_steps".to_string(),
            ElectroMinParam::MAX_SCF_CYCLES(_) => "max_scf_cycles".to_string(),
            ElectroMinParam::MAX_SD_STEPS => "max_sd_steps".to_string(),
            ElectroMinParam::METALS_METHOD(_) => "metals_method".to_string(),
            ElectroMinParam::NUM_DUMP_CYCLES(_) => "num_dump_cycles".to_string(),
            ElectroMinParam::SMEARING_SCHEME => "smearing_scheme".to_string(),
            ElectroMinParam::SMEARING_WIDTH(_) => "smearing_width".to_string(),
            ElectroMinParam::SPIN_FIX(_) => "spin_fix".to_string(),
        }
    }

    fn value(&self) -> String {
        match self {
            ElectroMinParam::EFERMI_TOL => todo!(),
            ElectroMinParam::ELEC_CONVERGENCE_WIN => todo!(),
            ElectroMinParam::ELEC_DUMP_FILE => todo!(),
            ElectroMinParam::ELEC_EIGENVALUE_TOL => todo!(),
            ElectroMinParam::ELEC_ENERGY_TOL(f) => format!("{f:18.15e}"),
            ElectroMinParam::ELEC_RESTORE_FILE => todo!(),
            ElectroMinParam::ELECTRONIC_MINIMIZER => todo!(),
            ElectroMinParam::FIX_OCCUPANCY(b) => format!("{b}"),
            ElectroMinParam::MAX_CG_STEPS => todo!(),
            ElectroMinParam::MAX_SCF_CYCLES(i) => format!("{i:>7}"),
            ElectroMinParam::MAX_SD_STEPS => todo!(),
            ElectroMinParam::METALS_METHOD(mm) => format!("{mm}"),
            ElectroMinParam::NUM_DUMP_CYCLES(i) => format!("{i:>3}"),
            ElectroMinParam::SMEARING_SCHEME => todo!(),
            ElectroMinParam::SMEARING_WIDTH(f) => format!("{f:24.15}"),
            ElectroMinParam::SPIN_FIX(i) => format!("{i}"),
        }
    }
}

impl Display for ElectroMinParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.output())
    }
}

impl SectionDisplay for ElectroMinParamSection {
    fn options(&self) -> &[impl Display] {
        &self.params
    }
}

impl Display for ElectroMinParamSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.section_content())
    }
}

#[cfg(test)]
mod test {

    use super::ElectroMinParamSection;

    #[test]
    fn test_em_param() {
        let dm_param = ElectroMinParamSection::default();
        println!("{dm_param}");
        let edft_param = ElectroMinParamSection::with_edft();
        println!("{edft_param}");
    }
}
