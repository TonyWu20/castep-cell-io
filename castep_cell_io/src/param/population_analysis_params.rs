use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromCellFile, ToCellFile, ToCell, FromKeyValue};
use super::population_analysis::*;

/// Population Analysis parameters for CASTEP calculations
///
/// This group contains settings that control population analysis calculations,
/// including Mulliken analysis, bond order analysis, and density of states weighting.
#[derive(Debug, Clone, Default, Builder)]
pub struct PopulationAnalysisParams {
    pub pdos_calculate_weights: Option<PdosCalculateWeights>,
    pub popn_bond_cutoff: Option<PopnBondCutoff>,
    pub popn_calculate: Option<PopnCalculate>,
    pub popn_write: Option<PopnWrite>,
}

impl PopulationAnalysisParams {
    /// Validates intra-group constraints for population analysis parameters
    pub fn validate(self) -> Result<Self, String> {
        // No specific intra-group validation needed for population analysis params
        Ok(self)
    }
}

impl FromCellFile for PopulationAnalysisParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_pdos_calculate_weights(PdosCalculateWeights::from_cells(tokens).ok().flatten())
            .maybe_popn_bond_cutoff(PopnBondCutoff::from_cells(tokens).ok().flatten())
            .maybe_popn_calculate(PopnCalculate::from_cells(tokens).ok().flatten())
            .maybe_popn_write(PopnWrite::from_cells(tokens).ok().flatten())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for PopulationAnalysisParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.pdos_calculate_weights { cells.push(v.to_cell()); }
        if let Some(v) = &self.popn_bond_cutoff { cells.push(v.to_cell()); }
        if let Some(v) = &self.popn_calculate { cells.push(v.to_cell()); }
        if let Some(v) = &self.popn_write { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_construction() {
        let params = PopulationAnalysisParams::default();
        assert!(params.pdos_calculate_weights.is_none());
        assert!(params.popn_write.is_none());
    }

    #[test]
    fn test_builder_construction() {
        let params = PopulationAnalysisParams::builder().build();
        assert!(params.pdos_calculate_weights.is_none());
    }

    #[test]
    fn test_validate_empty() {
        let params = PopulationAnalysisParams::default();
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_to_cell_file_empty() {
        let params = PopulationAnalysisParams::default();
        let cells = params.to_cell_file();
        assert_eq!(cells.len(), 0);
    }
}
