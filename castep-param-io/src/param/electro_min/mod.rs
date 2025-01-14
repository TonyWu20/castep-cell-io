use crate::param::KeywordDisplay;
use castep_param_derive::ParamDisplay;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

mod efermi_tol;
mod elec_convergence_win;
mod elec_dump_file;
mod elec_eigenvalue_tol;
mod elec_energy_tol;
mod elec_restore_file;
mod electronic_minimizer;
mod fix_occupancy;
mod max_cg_steps;
mod max_scf_cycles;
mod max_sd_steps;
mod metals_method;
mod num_dump_cycles;
mod num_occ_cycles;
mod smearing_scheme;
mod smearing_width;
mod spin_fix;

pub use efermi_tol::*;
pub use elec_convergence_win::ElecConvergenceWin;
pub use elec_dump_file::ElecDumpFile;
pub use elec_eigenvalue_tol::*;
pub use elec_energy_tol::*;
pub use elec_restore_file::ElecRestoreFile;
pub use electronic_minimizer::ElectronicMinimizer;
pub use fix_occupancy::FixOccupancy;
pub use max_cg_steps::MaxCgSteps;
pub use max_scf_cycles::MaxScfCycles;
pub use max_sd_steps::MaxSdSteps;
pub use metals_method::MetalsMethod;
pub use num_dump_cycles::NumDumpCycles;
pub use num_occ_cycles::NumOccCycles;
pub use smearing_scheme::SmearingScheme;
pub use smearing_width::SmearingWidth;
pub use spin_fix::SpinFix;

#[derive(
    Debug, Clone, ParamDisplay, Builder, Default, PartialEq, PartialOrd, Serialize, Deserialize,
)]
#[builder(setter(into, strip_option), default)]
pub struct ElectroMinimization {
    pub efermi_tol: Option<EFermiTol>,
    pub elec_convergence_win: Option<ElecConvergenceWin>,
    #[param_display(use_ref = true)]
    pub elec_dump_file: Option<ElecDumpFile>,
    pub elec_eigenvalue_tol: Option<ElecEigenvalueTol>,
    pub elec_energy_tol: Option<ElecEnergyTol>,
    #[param_display(use_ref = true)]
    pub elec_restore_file: Option<ElecRestoreFile>,
    pub electronic_minimizer: Option<ElectronicMinimizer>,
    pub fix_occupancy: Option<FixOccupancy>,
    pub max_cg_steps: Option<MaxCgSteps>,
    pub max_scf_cycles: Option<MaxScfCycles>,
    pub max_sd_steps: Option<MaxSdSteps>,
    pub metals_method: Option<MetalsMethod>,
    pub num_dump_cycles: Option<NumDumpCycles>,
    pub num_occ_cycles: Option<NumOccCycles>,
    pub smearing_scheme: Option<SmearingScheme>,
    pub smearing_width: Option<SmearingWidth>,
    pub spin_fix: Option<SpinFix>,
}

#[cfg(test)]
mod test {
    use crate::param::electro_min::MetalsMethod;

    use super::ElectroMinimizationBuilder;

    #[test]
    fn electro_min() {
        let p = ElectroMinimizationBuilder::default()
            .elec_energy_tol(1e-6)
            .elec_eigenvalue_tol(1e-6)
            .metals_method(MetalsMethod::DM)
            .max_scf_cycles(6000)
            .build()
            .unwrap();
        println!("{p}")
    }
}
