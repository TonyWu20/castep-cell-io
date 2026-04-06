use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromCellFile, ToCellFile, ToCell, FromKeyValue};
use super::phonon::*;

/// Phonon calculation parameters for CASTEP calculations
///
/// This group contains settings that control phonon calculations,
/// including Born charge calculations, convergence criteria, and phonon methods.
#[derive(Debug, Clone, Default, Builder)]
pub struct PhononParams {
    pub born_charge_sum_rule: Option<BornChargeSumRule>,
    pub calculate_born_charges: Option<CalculateBornCharges>,
    pub phonon_calc_lo_to_splitting: Option<PhononCalcLoToSplitting>,
    pub phonon_convergence_win: Option<PhononConvergenceWin>,
    pub phonon_energy_tol: Option<PhononEnergyTol>,
    pub phonon_fine_method: Option<PhononFineMethod>,
    pub phonon_finite_disp: Option<PhononFiniteDisp>,
    pub phonon_force_constant_cutoff: Option<PhononForceConstantCutoff>,
    pub phonon_max_cg_steps: Option<PhononMaxCgSteps>,
    pub phonon_max_cycles: Option<PhononMaxCycles>,
    pub phonon_method: Option<PhononMethod>,
    pub phonon_sum_rule: Option<PhononSumRule>,
}

impl PhononParams {
    /// Validates intra-group constraints for phonon parameters
    pub fn validate(self) -> Result<Self, String> {
        // No specific intra-group validation needed for phonon params
        Ok(self)
    }
}

impl FromCellFile for PhononParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_born_charge_sum_rule(BornChargeSumRule::from_cells(tokens).ok().flatten())
            .maybe_calculate_born_charges(CalculateBornCharges::from_cells(tokens).ok().flatten())
            .maybe_phonon_calc_lo_to_splitting(PhononCalcLoToSplitting::from_cells(tokens).ok().flatten())
            .maybe_phonon_convergence_win(PhononConvergenceWin::from_cells(tokens).ok().flatten())
            .maybe_phonon_energy_tol(PhononEnergyTol::from_cells(tokens).ok().flatten())
            .maybe_phonon_fine_method(PhononFineMethod::from_cells(tokens).ok().flatten())
            .maybe_phonon_finite_disp(PhononFiniteDisp::from_cells(tokens).ok().flatten())
            .maybe_phonon_force_constant_cutoff(PhononForceConstantCutoff::from_cells(tokens).ok().flatten())
            .maybe_phonon_max_cg_steps(PhononMaxCgSteps::from_cells(tokens).ok().flatten())
            .maybe_phonon_max_cycles(PhononMaxCycles::from_cells(tokens).ok().flatten())
            .maybe_phonon_method(PhononMethod::from_cells(tokens).ok().flatten())
            .maybe_phonon_sum_rule(PhononSumRule::from_cells(tokens).ok().flatten())
            .build()
            .validate()
            .map_err(Error::Message)
    }
}

impl ToCellFile for PhononParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.born_charge_sum_rule { cells.push(v.to_cell()); }
        if let Some(v) = &self.calculate_born_charges { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_calc_lo_to_splitting { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_convergence_win { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_energy_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_fine_method { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_finite_disp { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_force_constant_cutoff { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_max_cg_steps { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_max_cycles { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_method { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_sum_rule { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phonon_params_default() {
        let params = PhononParams::default();
        assert!(params.born_charge_sum_rule.is_none());
        assert!(params.phonon_max_cycles.is_none());
    }

    #[test]
    fn test_phonon_params_builder() {
        let params = PhononParams::builder()
            .maybe_phonon_max_cycles(Some(PhononMaxCycles(50)))
            .build();
        assert_eq!(params.phonon_max_cycles.unwrap().0, 50);
    }

    #[test]
    fn test_phonon_params_validate() {
        let params = PhononParams::default();
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_phonon_params_to_cell_file() {
        let params = PhononParams::builder()
            .maybe_phonon_max_cycles(Some(PhononMaxCycles(30)))
            .build();
        let cells = params.to_cell_file();
        assert_eq!(cells.len(), 1);
    }
}
