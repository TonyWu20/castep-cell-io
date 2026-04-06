use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromCellFile, ToCellFile, ToCell, FromKeyValue};
use super::nmr::*;

/// NMR parameters for CASTEP calculations
///
/// This group contains settings that control NMR (magnetic resonance) calculations,
/// including convergence tolerances, iteration limits, and calculation methods.
#[derive(Debug, Clone, Default, Builder)]
pub struct NmrParams {
    pub magres_conv_tol: Option<MagresConvTol>,
    pub magres_max_cg_steps: Option<MagresMaxCgSteps>,
    pub magres_method: Option<MagresMethod>,
    pub magres_task: Option<MagresTask>,
}

impl NmrParams {
    /// Validates intra-group constraints for NMR parameters
    pub fn validate(self) -> Result<Self, String> {
        // No specific intra-group validation needed for NMR params
        Ok(self)
    }
}

impl FromCellFile for NmrParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_magres_conv_tol(MagresConvTol::from_cells(tokens).ok().flatten())
            .maybe_magres_max_cg_steps(MagresMaxCgSteps::from_cells(tokens).ok().flatten())
            .maybe_magres_method(MagresMethod::from_cells(tokens).ok().flatten())
            .maybe_magres_task(MagresTask::from_cells(tokens).ok().flatten())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for NmrParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.magres_conv_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.magres_max_cg_steps { cells.push(v.to_cell()); }
        if let Some(v) = &self.magres_method { cells.push(v.to_cell()); }
        if let Some(v) = &self.magres_task { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nmr_params_default() {
        let params = NmrParams::default();
        assert!(params.magres_conv_tol.is_none());
        assert!(params.magres_max_cg_steps.is_none());
        assert!(params.magres_method.is_none());
        assert!(params.magres_task.is_none());
    }

    #[test]
    fn test_nmr_params_builder() {
        let params = NmrParams::builder().build();
        assert!(params.magres_conv_tol.is_none());
        assert!(params.magres_max_cg_steps.is_none());
    }

    #[test]
    fn test_nmr_params_to_cell_file_empty() {
        let params = NmrParams::default();
        let cells = params.to_cell_file();
        assert_eq!(cells.len(), 0);
    }
}
