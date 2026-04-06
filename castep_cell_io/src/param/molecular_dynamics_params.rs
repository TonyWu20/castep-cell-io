use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromCellFile, ToCellFile, ToCell, FromKeyValue};
use super::molecular_dynamics::*;

/// Molecular Dynamics parameters for CASTEP calculations
///
/// This group contains settings that control molecular dynamics simulations,
/// including ensemble selection, temperature control, time stepping, and
/// electronic convergence criteria during MD.
#[derive(Debug, Clone, Default, Builder)]
pub struct MolecularDynamicsParams {
    pub md_barostat: Option<MdBarostat>,
    pub md_cell_t: Option<MdCellT>,
    pub md_damping_reset: Option<MdDampingReset>,
    pub md_damping_scheme: Option<MdDampingScheme>,
    pub md_delta_t: Option<MdDeltaT>,
    pub md_elec_convergence_win: Option<MdElecConvergenceWin>,
    pub md_elec_eigenvalue_tol: Option<MdElecEigenvalueTol>,
    pub md_elec_energy_tol: Option<MdElecEnergyTol>,
    pub md_ensemble: Option<MdEnsemble>,
    pub md_eqm_cell_t: Option<MdEqmCellT>,
    pub md_eqm_ion_t: Option<MdEqmIonT>,
    pub md_eqm_method: Option<MdEqmMethod>,
    pub md_eqm_t: Option<MdEqmT>,
    pub md_extrap: Option<MdExtrap>,
    pub md_extrap_fit: Option<MdExtrapFit>,
    pub md_ion_t: Option<MdIonT>,
    pub md_num_iter: Option<MdNumIter>,
    pub md_opt_damped_delta_t: Option<MdOptDampedDeltaT>,
    pub md_temperature: Option<MdTemperature>,
    pub md_thermostat: Option<MdThermostat>,
}

impl MolecularDynamicsParams {
    /// Validates intra-group constraints for molecular dynamics parameters
    pub fn validate(self) -> Result<Self, String> {
        // No specific intra-group validation needed for MD params
        Ok(self)
    }
}

impl FromCellFile for MolecularDynamicsParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_md_barostat(MdBarostat::from_cells(tokens).ok().flatten())
            .maybe_md_cell_t(MdCellT::from_cells(tokens).ok().flatten())
            .maybe_md_damping_reset(MdDampingReset::from_cells(tokens).ok().flatten())
            .maybe_md_damping_scheme(MdDampingScheme::from_cells(tokens).ok().flatten())
            .maybe_md_delta_t(MdDeltaT::from_cells(tokens).ok().flatten())
            .maybe_md_elec_convergence_win(MdElecConvergenceWin::from_cells(tokens).ok().flatten())
            .maybe_md_elec_eigenvalue_tol(MdElecEigenvalueTol::from_cells(tokens).ok().flatten())
            .maybe_md_elec_energy_tol(MdElecEnergyTol::from_cells(tokens).ok().flatten())
            .maybe_md_ensemble(MdEnsemble::from_cells(tokens).ok().flatten())
            .maybe_md_eqm_cell_t(MdEqmCellT::from_cells(tokens).ok().flatten())
            .maybe_md_eqm_ion_t(MdEqmIonT::from_cells(tokens).ok().flatten())
            .maybe_md_eqm_method(MdEqmMethod::from_cells(tokens).ok().flatten())
            .maybe_md_eqm_t(MdEqmT::from_cells(tokens).ok().flatten())
            .maybe_md_extrap(MdExtrap::from_cells(tokens).ok().flatten())
            .maybe_md_extrap_fit(MdExtrapFit::from_cells(tokens).ok().flatten())
            .maybe_md_ion_t(MdIonT::from_cells(tokens).ok().flatten())
            .maybe_md_num_iter(MdNumIter::from_cells(tokens).ok().flatten())
            .maybe_md_opt_damped_delta_t(MdOptDampedDeltaT::from_cells(tokens).ok().flatten())
            .maybe_md_temperature(MdTemperature::from_cells(tokens).ok().flatten())
            .maybe_md_thermostat(MdThermostat::from_cells(tokens).ok().flatten())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for MolecularDynamicsParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.md_barostat { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_cell_t { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_damping_reset { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_damping_scheme { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_delta_t { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_elec_convergence_win { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_elec_eigenvalue_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_elec_energy_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_ensemble { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_eqm_cell_t { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_eqm_ion_t { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_eqm_method { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_eqm_t { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_extrap { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_extrap_fit { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_ion_t { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_num_iter { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_opt_damped_delta_t { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_temperature { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_thermostat { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_fmt::parse;

    #[test]
    fn test_default_construction() {
        let params = MolecularDynamicsParams::default();
        assert!(params.md_barostat.is_none());
        assert!(params.md_ensemble.is_none());
        assert!(params.md_temperature.is_none());
    }

    #[test]
    fn test_builder_construction() {
        let params = MolecularDynamicsParams::builder().build();
        assert!(params.md_barostat.is_none());
    }

    #[test]
    fn test_validate_empty() {
        let params = MolecularDynamicsParams::default();
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_to_cell_file_empty() {
        let params = MolecularDynamicsParams::default();
        let cells = params.to_cell_file();
        assert_eq!(cells.len(), 0);
    }
}
