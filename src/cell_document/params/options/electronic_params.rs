use std::fmt::Display;

use super::{OptionDisplay, SectionDisplay};

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ElectronicParam {
    // CHARGE,
    // NBANDS,
    // NDOWN,
    // NELECTRONS,
    // NEXTRA_BANDS,
    // NUP,
    PERC_EXTRA_BANDS(f64),
    // SEDC_APPLY,
    // SEDC_D_G06,
    // SEDC_D_JCHS,
    // SEDC_D_TS,
    // SEDC_LAMBDA_OBS,
    // SEDC_N_OBS,
    // SEDC_S6_G06,
    // SEDC_S6_JCHS,
    // SEDC_SR_JCHS,
    // SEDC_SR_TS,
    // SEDC_SCHEME,
    SPIN(u32),
}

#[derive(Debug, Clone)]
pub struct ElectronicParamSection {
    params: Vec<ElectronicParam>,
}

impl ElectronicParamSection {
    pub fn new(params: Vec<ElectronicParam>) -> Self {
        Self { params }
    }
    pub fn with_spin_and_perc_extra_bands(spin: u32, perc_extra_band: f64) -> Self {
        Self::new(vec![
            ElectronicParam::PERC_EXTRA_BANDS(perc_extra_band), //default by us
            ElectronicParam::SPIN(spin),
        ])
    }

    pub fn params(&self) -> &[ElectronicParam] {
        &self.params
    }
}

impl OptionDisplay for ElectronicParam {
    fn tag(&self) -> String {
        match self {
            ElectronicParam::PERC_EXTRA_BANDS(_) => "perc_extra_bands".to_string(),
            ElectronicParam::SPIN(_) => "spin".to_string(),
        }
    }

    fn value(&self) -> String {
        match self {
            ElectronicParam::PERC_EXTRA_BANDS(f) => format!("{f}"),
            ElectronicParam::SPIN(u) => format!("{u}"),
        }
    }
}

impl Display for ElectronicParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.output())
    }
}

impl SectionDisplay for ElectronicParamSection {
    fn options(&self) -> &[impl Display] {
        self.params()
    }
}

impl Display for ElectronicParamSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.section_content())
    }
}

#[cfg(test)]
mod test {
    use super::ElectronicParamSection;

    #[test]
    fn test_elec_pm() {
        let pm = ElectronicParamSection::with_spin_and_perc_extra_bands(2, 72_f64);
        println!("{pm}");
    }
}
