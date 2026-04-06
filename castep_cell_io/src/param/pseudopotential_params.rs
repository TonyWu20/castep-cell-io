use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromCellFile, ToCellFile, ToCell, FromKeyValue};
use super::pseudopotential::*;

/// Pseudopotential parameters for CASTEP calculations
///
/// This group contains settings that control pseudopotential behavior,
/// including beta-phi type, nonlocal type, and relativistic treatment options.
#[derive(Debug, Clone, Default, Builder)]
pub struct PseudopotentialParams {
    pub pspot_beta_phi_type: Option<PspotBetaPhiType>,
    pub pspot_nonlocal_type: Option<PspotNonlocalType>,
    pub relativistic_treatment: Option<RelativisticTreatment>,
}

impl PseudopotentialParams {
    /// Validates intra-group constraints for pseudopotential parameters
    pub fn validate(self) -> Result<Self, String> {
        // No specific intra-group validation needed for pseudopotential params
        Ok(self)
    }
}

impl FromCellFile for PseudopotentialParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_pspot_beta_phi_type(PspotBetaPhiType::from_cells(tokens).ok().flatten())
            .maybe_pspot_nonlocal_type(PspotNonlocalType::from_cells(tokens).ok().flatten())
            .maybe_relativistic_treatment(RelativisticTreatment::from_cells(tokens).ok().flatten())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for PseudopotentialParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.pspot_beta_phi_type { cells.push(v.to_cell()); }
        if let Some(v) = &self.pspot_nonlocal_type { cells.push(v.to_cell()); }
        if let Some(v) = &self.relativistic_treatment { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_construction() {
        let params = PseudopotentialParams::default();
        assert!(params.pspot_beta_phi_type.is_none());
        assert!(params.relativistic_treatment.is_none());
    }

    #[test]
    fn test_builder_construction() {
        let params = PseudopotentialParams::builder().build();
        assert!(params.pspot_beta_phi_type.is_none());
    }

    #[test]
    fn test_validate_empty() {
        let params = PseudopotentialParams::default();
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_to_cell_file_empty() {
        let params = PseudopotentialParams::default();
        let cells = params.to_cell_file();
        assert_eq!(cells.len(), 0);
    }
}
