use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromCellFile, ToCellFile, ToCell, FromKeyValue};
use super::electronic_minimisation::*;

/// Electronic minimisation parameters for CASTEP calculations
///
/// This group contains settings that control the electronic minimization process,
/// including SCF convergence criteria, minimizer selection, and smearing options.
#[derive(Debug, Clone, Default, Builder)]
pub struct ElectronicMinimisationParams {
    pub efermi_tol: Option<EFermiTol>,
    pub elec_convergence_win: Option<ElecConvergenceWin>,
    pub elec_dump_file: Option<ElecDumpFile>,
    pub elec_eigenvalue_tol: Option<ElecEigenvalueTol>,
    pub elec_energy_tol: Option<ElecEnergyTol>,
    pub elec_restore_file: Option<ElecRestoreFile>,
    pub electronic_minimizer: Option<ElectronicMinimizer>,
    pub fix_occupancy: Option<FixOccupancy>,
    pub max_cg_steps: Option<MaxCgSteps>,
    pub max_scf_cycles: Option<MaxScfCycles>,
    pub max_sd_steps: Option<MaxSdSteps>,
    pub metals_method: Option<MetalsMethod>,
    pub num_dump_cycles: Option<NumDumpCycles>,
    pub smearing_scheme: Option<SmearingScheme>,
    pub smearing_width: Option<SmearingWidth>,
    pub spin_fix: Option<SpinFix>,
}

impl ElectronicMinimisationParams {
    /// Validates intra-group constraints for electronic minimisation parameters
    pub fn validate(self) -> Result<Self, String> {
        Ok(self)
    }
}

impl FromCellFile for ElectronicMinimisationParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_efermi_tol(EFermiTol::from_cells(tokens).ok().flatten())
            .maybe_elec_convergence_win(ElecConvergenceWin::from_cells(tokens).ok().flatten())
            .maybe_elec_dump_file(ElecDumpFile::from_cells(tokens).ok().flatten())
            .maybe_elec_eigenvalue_tol(ElecEigenvalueTol::from_cells(tokens).ok().flatten())
            .maybe_elec_energy_tol(ElecEnergyTol::from_cells(tokens).ok().flatten())
            .maybe_elec_restore_file(ElecRestoreFile::from_cells(tokens).ok().flatten())
            .maybe_electronic_minimizer(ElectronicMinimizer::from_cells(tokens).ok().flatten())
            .maybe_fix_occupancy(FixOccupancy::from_cells(tokens).ok().flatten())
            .maybe_max_cg_steps(MaxCgSteps::from_cells(tokens).ok().flatten())
            .maybe_max_scf_cycles(MaxScfCycles::from_cells(tokens).ok().flatten())
            .maybe_max_sd_steps(MaxSdSteps::from_cells(tokens).ok().flatten())
            .maybe_metals_method(MetalsMethod::from_cells(tokens).ok().flatten())
            .maybe_num_dump_cycles(NumDumpCycles::from_cells(tokens).ok().flatten())
            .maybe_smearing_scheme(SmearingScheme::from_cells(tokens).ok().flatten())
            .maybe_smearing_width(SmearingWidth::from_cells(tokens).ok().flatten())
            .maybe_spin_fix(SpinFix::from_cells(tokens).ok().flatten())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for ElectronicMinimisationParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.efermi_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.elec_convergence_win { cells.push(v.to_cell()); }
        if let Some(v) = &self.elec_dump_file { cells.push(v.to_cell()); }
        if let Some(v) = &self.elec_eigenvalue_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.elec_energy_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.elec_restore_file { cells.push(v.to_cell()); }
        if let Some(v) = &self.electronic_minimizer { cells.push(v.to_cell()); }
        if let Some(v) = &self.fix_occupancy { cells.push(v.to_cell()); }
        if let Some(v) = &self.max_cg_steps { cells.push(v.to_cell()); }
        if let Some(v) = &self.max_scf_cycles { cells.push(v.to_cell()); }
        if let Some(v) = &self.max_sd_steps { cells.push(v.to_cell()); }
        if let Some(v) = &self.metals_method { cells.push(v.to_cell()); }
        if let Some(v) = &self.num_dump_cycles { cells.push(v.to_cell()); }
        if let Some(v) = &self.smearing_scheme { cells.push(v.to_cell()); }
        if let Some(v) = &self.smearing_width { cells.push(v.to_cell()); }
        if let Some(v) = &self.spin_fix { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creates_empty_struct() {
        let params = ElectronicMinimisationParams::builder().build();
        assert!(params.efermi_tol.is_none());
        assert!(params.max_scf_cycles.is_none());
    }

    #[test]
    fn test_builder_with_single_field() {
        let params = ElectronicMinimisationParams::builder()
            .maybe_max_scf_cycles(Some(MaxScfCycles(100)))
            .build();
        assert_eq!(params.max_scf_cycles.unwrap().0, 100);
    }

    #[test]
    fn test_to_cell_file_empty() {
        let params = ElectronicMinimisationParams::builder().build();
        let cells = params.to_cell_file();
        assert!(cells.is_empty());
    }

    #[test]
    fn test_to_cell_file_with_fields() {
        let params = ElectronicMinimisationParams::builder()
            .maybe_max_scf_cycles(Some(MaxScfCycles(50)))
            .maybe_max_cg_steps(Some(MaxCgSteps(100)))
            .build();
        let cells = params.to_cell_file();
        assert_eq!(cells.len(), 2);
    }

    #[test]
    fn test_validate_succeeds() {
        let params = ElectronicMinimisationParams::builder().build();
        assert!(params.validate().is_ok());
    }
}
