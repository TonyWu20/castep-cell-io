use std::fmt::Display;

use crate::cell_document::units::charge_units::ChargeUnits;

use opt_strategy::OptStrategy;

use super::{OptionDisplay, ParamSectionDisplay};

mod opt_strategy;
mod task;

pub use task::CastepTask;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum GeneralParam {
    BACKUP_INTERVAL(u32),
    CALCULATE_STRESS(bool),
    CALCULATE_DENSDIFF(bool),
    CALCULATE_ELF(bool),
    CALCULATE_HIRSHFELD(bool),
    CHARGE_UNIT(ChargeUnits),
    CHECKPOINT(String),
    COMMENT(String),
    CONTINUATION(String),
    DATA_DISTRIBUTION, // TODO: impl struct
    IPRINT,            // TODO: Use enum to represent 0-3
    NUM_BACKUP_ITER(u32),
    OPT_STRATEGY(OptStrategy),
    PAGE_WVFNS(i32),
    PRINT_CLOCK(bool),
    PRINT_MEMORY_USAGE(bool),
    RAND_SEED(i32),
    REUSE(String),
    RUN_TIME(i32),
    STOP,
    TASK(CastepTask),
    WRITE_CHECKPOINT, // TODO: impl struct
    WRITE_FORMATTED_DENSITY(bool),
    WRITE_FORMATTED_ELF(bool),
    WRITE_FORMATTED_POTENTIAL(bool),
    WRITE_ORBITALS(bool),
}

#[derive(Debug, Clone)]
pub struct GeneralParamSection {
    params: Vec<GeneralParam>,
}

impl GeneralParamSection {
    pub fn new(params: Vec<GeneralParam>) -> Self {
        Self { params }
    }
    pub fn band_structure() -> Self {
        Self::new(vec![
            GeneralParam::TASK(CastepTask::BandStructure),
            GeneralParam::COMMENT("CASTEP calculation from Materials Studio".to_string()),
            GeneralParam::CONTINUATION("default".to_string()),
            GeneralParam::OPT_STRATEGY(OptStrategy::Speed),
            GeneralParam::PAGE_WVFNS(0),
            GeneralParam::CALCULATE_ELF(false),
            GeneralParam::CALCULATE_STRESS(false),
            GeneralParam::CALCULATE_HIRSHFELD(false),
            GeneralParam::CALCULATE_DENSDIFF(false),
        ])
    }
}

impl Default for GeneralParamSection {
    fn default() -> Self {
        Self::new(vec![
            GeneralParam::TASK(CastepTask::GeometryOptimization),
            GeneralParam::COMMENT("CASTEP calculation from Materials Studio".to_string()),
            GeneralParam::OPT_STRATEGY(OptStrategy::Speed),
            GeneralParam::PAGE_WVFNS(0),
            GeneralParam::CALCULATE_ELF(false),
            GeneralParam::CALCULATE_STRESS(false),
            GeneralParam::CALCULATE_HIRSHFELD(true),
            GeneralParam::CALCULATE_DENSDIFF(false),
        ])
    }
}

impl OptionDisplay for GeneralParam {
    fn tag(&self) -> String {
        match self {
            GeneralParam::BACKUP_INTERVAL(_) => "backup_interval".to_string(),
            GeneralParam::CALCULATE_STRESS(_) => "calculate_stress".to_string(),
            GeneralParam::CALCULATE_DENSDIFF(_) => "calculate_densdiff".to_string(),
            GeneralParam::CALCULATE_ELF(_) => "calculate_ELF".to_string(),
            GeneralParam::CALCULATE_HIRSHFELD(_) => "calculate_hirshfeld".to_string(),
            GeneralParam::CHARGE_UNIT(_) => "charge_unit".to_string(),
            GeneralParam::CHECKPOINT(_) => "checkpoint".to_string(),
            GeneralParam::COMMENT(_) => "comment".to_string(),
            GeneralParam::CONTINUATION(_) => "continuation".to_string(),
            GeneralParam::DATA_DISTRIBUTION => "data_distribution".to_string(),
            GeneralParam::IPRINT => "iprint".to_string(),
            GeneralParam::NUM_BACKUP_ITER(_) => "num_backup_iter".to_string(),
            GeneralParam::OPT_STRATEGY(_) => "opt_strategy".to_string(),
            GeneralParam::PAGE_WVFNS(_) => "page_wvfns".to_string(),
            GeneralParam::PRINT_CLOCK(_) => "print_clock".to_string(),
            GeneralParam::PRINT_MEMORY_USAGE(_) => "print_memory_usage".to_string(),
            GeneralParam::RAND_SEED(_) => "rand_seed".to_string(),
            GeneralParam::REUSE(_) => "reuse".to_string(),
            GeneralParam::RUN_TIME(_) => "run_time".to_string(),
            GeneralParam::STOP => "stop".to_string(),
            GeneralParam::TASK(_) => "task".to_string(),
            GeneralParam::WRITE_CHECKPOINT => "write_checkpoint".to_string(),
            GeneralParam::WRITE_FORMATTED_DENSITY(_) => "write_formatted_density".to_string(),
            GeneralParam::WRITE_FORMATTED_ELF(_) => "write_formatted_elf".to_string(),
            GeneralParam::WRITE_FORMATTED_POTENTIAL(_) => "write_formatted_potential".to_string(),
            GeneralParam::WRITE_ORBITALS(_) => "write_orbitals".to_string(),
        }
    }

    fn value(&self) -> String {
        match self {
            GeneralParam::BACKUP_INTERVAL(u) => format!("{u}"),
            GeneralParam::CALCULATE_STRESS(b) => format!("{b}"),
            GeneralParam::CALCULATE_DENSDIFF(b) => format!("{b}"),
            GeneralParam::CALCULATE_ELF(b) => format!("{b}"),
            GeneralParam::CALCULATE_HIRSHFELD(b) => format!("{b}"),
            GeneralParam::CHARGE_UNIT(cg) => format!("{cg}"),
            GeneralParam::CHECKPOINT(s) => s.to_string(),
            GeneralParam::COMMENT(s) => s.to_string(),
            GeneralParam::CONTINUATION(s) => s.to_string(),
            GeneralParam::DATA_DISTRIBUTION => todo!(),
            GeneralParam::IPRINT => todo!(),
            GeneralParam::NUM_BACKUP_ITER(u) => format!("{u}"),
            GeneralParam::OPT_STRATEGY(opt) => opt.value(),
            GeneralParam::PAGE_WVFNS(i) => format!("{i}"),
            GeneralParam::PRINT_CLOCK(b) => format!("{b}"),
            GeneralParam::PRINT_MEMORY_USAGE(b) => format!("{b}"),
            GeneralParam::RAND_SEED(i) => format!("{i}"),
            GeneralParam::REUSE(s) => s.to_string(),
            GeneralParam::RUN_TIME(i) => format!("{i}"),
            GeneralParam::STOP => String::new(),
            GeneralParam::TASK(t) => format!("{t}"),
            GeneralParam::WRITE_CHECKPOINT => todo!(),
            GeneralParam::WRITE_FORMATTED_DENSITY(b) => format!("{b}"),
            GeneralParam::WRITE_FORMATTED_ELF(b) => format!("{b}"),
            GeneralParam::WRITE_FORMATTED_POTENTIAL(b) => format!("{b}"),
            GeneralParam::WRITE_ORBITALS(b) => format!("{b}"),
        }
    }
    fn output(&self) -> String {
        match self {
            GeneralParam::STOP => "STOP".to_string(),
            _ => format!("{} : {}", self.tag(), self.value()),
        }
    }
}

impl Display for GeneralParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.output())
    }
}

impl ParamSectionDisplay for GeneralParamSection {
    fn options(&self) -> &[impl Display] {
        &self.params
    }
}

impl Display for GeneralParamSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.section_content())
    }
}

#[cfg(test)]
mod test {
    use super::GeneralParamSection;

    #[test]
    fn test_general_pm() {
        let pm = GeneralParamSection::default();
        let bs_pm = GeneralParamSection::band_structure();
        println!("{pm}");
        println!("{bs_pm}");
    }
}
