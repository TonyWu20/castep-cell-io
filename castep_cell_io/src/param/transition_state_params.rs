use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromCellFile, ToCellFile, ToCell, FromKeyValue};
use super::transition_state::*;

/// Transition State parameters for CASTEP calculations
///
/// This group contains settings that control transition state search calculations,
/// including method selection, convergence criteria, and path point management.
#[derive(Debug, Clone, Default, Builder)]
pub struct TransitionStateParams {
    pub tssearch_method: Option<TssearchMethod>,
    pub tssearch_lstqst_protocol: Option<TssearchLstqstProtocol>,
    pub tssearch_cg_max_iter: Option<TssearchCgMaxIter>,
    pub tssearch_max_path_points: Option<TssearchMaxPathPoints>,
    pub tssearch_qst_max_iter: Option<TssearchQstMaxIter>,
    pub tssearch_disp_tol: Option<TssearchDispTol>,
    pub tssearch_energy_tol: Option<TssearchEnergyTol>,
    pub tssearch_force_tol: Option<TssearchForceTol>,
}

impl TransitionStateParams {
    /// Validates intra-group constraints for transition state parameters
    pub fn validate(self) -> Result<Self, String> {
        // No specific intra-group validation needed for transition state params
        Ok(self)
    }
}

impl FromCellFile for TransitionStateParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_tssearch_method(TssearchMethod::from_cells(tokens).ok().flatten())
            .maybe_tssearch_lstqst_protocol(TssearchLstqstProtocol::from_cells(tokens).ok().flatten())
            .maybe_tssearch_cg_max_iter(TssearchCgMaxIter::from_cells(tokens).ok().flatten())
            .maybe_tssearch_max_path_points(TssearchMaxPathPoints::from_cells(tokens).ok().flatten())
            .maybe_tssearch_qst_max_iter(TssearchQstMaxIter::from_cells(tokens).ok().flatten())
            .maybe_tssearch_disp_tol(TssearchDispTol::from_cells(tokens).ok().flatten())
            .maybe_tssearch_energy_tol(TssearchEnergyTol::from_cells(tokens).ok().flatten())
            .maybe_tssearch_force_tol(TssearchForceTol::from_cells(tokens).ok().flatten())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for TransitionStateParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.tssearch_method { cells.push(v.to_cell()); }
        if let Some(v) = &self.tssearch_lstqst_protocol { cells.push(v.to_cell()); }
        if let Some(v) = &self.tssearch_cg_max_iter { cells.push(v.to_cell()); }
        if let Some(v) = &self.tssearch_max_path_points { cells.push(v.to_cell()); }
        if let Some(v) = &self.tssearch_qst_max_iter { cells.push(v.to_cell()); }
        if let Some(v) = &self.tssearch_disp_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.tssearch_energy_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.tssearch_force_tol { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_construction() {
        let params = TransitionStateParams::default();
        assert!(params.tssearch_method.is_none());
        assert!(params.tssearch_force_tol.is_none());
    }

    #[test]
    fn test_builder_construction() {
        let params = TransitionStateParams::builder().build();
        assert!(params.tssearch_method.is_none());
    }

    #[test]
    fn test_validate_empty() {
        let params = TransitionStateParams::default();
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_to_cell_file_empty() {
        let params = TransitionStateParams::default();
        let cells = params.to_cell_file();
        assert_eq!(cells.len(), 0);
    }
}
