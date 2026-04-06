use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromCellFile, ToCellFile, ToCell, FromKeyValue};
use super::electronic_excitations::*;

/// Electronic Excitations parameters for CASTEP calculations
///
/// This group contains settings that control excited state calculations,
/// including TDDFT and spectral property calculations.
#[derive(Debug, Clone, Default, Builder)]
pub struct ElectronicExcitationsParams {
    pub spectral_task: Option<SpectralTask>,
    pub tddft_position_method: Option<TddftPositionMethod>,
    pub tddft_num_states: Option<TddftNumStates>,
    pub tddft_selected_state: Option<TddftSelectedState>,
}

impl ElectronicExcitationsParams {
    /// Validates intra-group constraints for electronic excitations parameters
    pub fn validate(self) -> Result<Self, String> {
        // No specific intra-group validation needed for electronic excitations params
        Ok(self)
    }
}

impl FromCellFile for ElectronicExcitationsParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_spectral_task(SpectralTask::from_cells(tokens).ok().flatten())
            .maybe_tddft_position_method(TddftPositionMethod::from_cells(tokens).ok().flatten())
            .maybe_tddft_num_states(TddftNumStates::from_cells(tokens).ok().flatten())
            .maybe_tddft_selected_state(TddftSelectedState::from_cells(tokens).ok().flatten())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for ElectronicExcitationsParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.spectral_task { cells.push(v.to_cell()); }
        if let Some(v) = &self.tddft_position_method { cells.push(v.to_cell()); }
        if let Some(v) = &self.tddft_num_states { cells.push(v.to_cell()); }
        if let Some(v) = &self.tddft_selected_state { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_construction() {
        let params = ElectronicExcitationsParams::default();
        assert!(params.spectral_task.is_none());
        assert!(params.tddft_selected_state.is_none());
    }

    #[test]
    fn test_builder_construction() {
        let params = ElectronicExcitationsParams::builder().build();
        assert!(params.spectral_task.is_none());
    }

    #[test]
    fn test_validate_empty() {
        let params = ElectronicExcitationsParams::default();
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_to_cell_file_empty() {
        let params = ElectronicExcitationsParams::default();
        let cells = params.to_cell_file();
        assert_eq!(cells.len(), 0);
    }
}
