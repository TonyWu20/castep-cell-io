use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromCellFile, ToCellFile, ToCell, FromKeyValue};
use super::efield::*;

/// Electric Field parameters for CASTEP calculations
///
/// This group contains settings that control linear response to electric field calculations,
/// including convergence criteria, maximum iterations, and nonlinear response options.
#[derive(Debug, Clone, Default, Builder)]
pub struct ElectricFieldParams {
    pub efield_calc_ion_permittivity: Option<EfieldCalcIonPermittivity>,
    pub efield_calculate_nonlinear: Option<EfieldCalculateNonlinear>,
    pub efield_convergence_win: Option<EfieldConvergenceWin>,
    pub efield_energy_tol: Option<EfieldEnergyTol>,
    pub efield_ignore_mol_modes: Option<EfieldIgnoreMolModes>,
    pub efield_max_cg_steps: Option<EfieldMaxCgSteps>,
    pub efield_max_cycles: Option<EfieldMaxCycles>,
}

impl ElectricFieldParams {
    /// Validates intra-group constraints for electric field parameters
    pub fn validate(self) -> Result<Self, String> {
        // No specific intra-group validation needed for electric field params
        Ok(self)
    }
}

impl FromCellFile for ElectricFieldParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_efield_calc_ion_permittivity(EfieldCalcIonPermittivity::from_cells(tokens).ok().flatten())
            .maybe_efield_calculate_nonlinear(EfieldCalculateNonlinear::from_cells(tokens).ok().flatten())
            .maybe_efield_convergence_win(EfieldConvergenceWin::from_cells(tokens).ok().flatten())
            .maybe_efield_energy_tol(EfieldEnergyTol::from_cells(tokens).ok().flatten())
            .maybe_efield_ignore_mol_modes(EfieldIgnoreMolModes::from_cells(tokens).ok().flatten())
            .maybe_efield_max_cg_steps(EfieldMaxCgSteps::from_cells(tokens).ok().flatten())
            .maybe_efield_max_cycles(EfieldMaxCycles::from_cells(tokens).ok().flatten())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for ElectricFieldParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.efield_calc_ion_permittivity { cells.push(v.to_cell()); }
        if let Some(v) = &self.efield_calculate_nonlinear { cells.push(v.to_cell()); }
        if let Some(v) = &self.efield_convergence_win { cells.push(v.to_cell()); }
        if let Some(v) = &self.efield_energy_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.efield_ignore_mol_modes { cells.push(v.to_cell()); }
        if let Some(v) = &self.efield_max_cg_steps { cells.push(v.to_cell()); }
        if let Some(v) = &self.efield_max_cycles { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_construction() {
        let params = ElectricFieldParams::default();
        assert!(params.efield_calc_ion_permittivity.is_none());
        assert!(params.efield_max_cycles.is_none());
    }

    #[test]
    fn test_builder_construction() {
        let params = ElectricFieldParams::builder().build();
        assert!(params.efield_calc_ion_permittivity.is_none());
    }

    #[test]
    fn test_validate_empty() {
        let params = ElectricFieldParams::default();
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_to_cell_file_empty() {
        let params = ElectricFieldParams::default();
        let cells = params.to_cell_file();
        assert_eq!(cells.len(), 0);
    }
}
