use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromCellFile, ToCellFile, ToCell, FromKeyValue};
use super::optics::*;

/// Optics parameters for CASTEP calculations
///
/// This group contains settings that control optical property calculations,
/// including exchange-correlation functional and band structure parameters
/// specific to optics calculations.
#[derive(Debug, Clone, Default, Builder)]
pub struct OpticsParams {
    pub optics_xc_functional: Option<OpticXcFunctional>,
    pub optics_nbands: Option<OpticsNbands>,
    pub optics_nextra_bands: Option<OpticsNextraBands>,
    pub optics_perc_extra_bands: Option<OpticsPercExtraBands>,
}

impl OpticsParams {
    /// Validates intra-group constraints for optics parameters
    pub fn validate(self) -> Result<Self, String> {
        // Check mutual exclusivity of optics_nbands, optics_nextra_bands, and optics_perc_extra_bands
        let count = [
            self.optics_nbands.is_some(),
            self.optics_nextra_bands.is_some(),
            self.optics_perc_extra_bands.is_some(),
        ].iter().filter(|&&x| x).count();

        if count > 1 {
            return Err("OPTICS_NBANDS, OPTICS_NEXTRA_BANDS, and OPTICS_PERC_EXTRA_BANDS are mutually exclusive. Only one may be specified.".into());
        }

        Ok(self)
    }
}

impl FromCellFile for OpticsParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_optics_xc_functional(OpticXcFunctional::from_cells(tokens).ok().flatten())
            .maybe_optics_nbands(OpticsNbands::from_cells(tokens).ok().flatten())
            .maybe_optics_nextra_bands(OpticsNextraBands::from_cells(tokens).ok().flatten())
            .maybe_optics_perc_extra_bands(OpticsPercExtraBands::from_cells(tokens).ok().flatten())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for OpticsParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.optics_xc_functional { cells.push(v.to_cell()); }
        if let Some(v) = &self.optics_nbands { cells.push(v.to_cell()); }
        if let Some(v) = &self.optics_nextra_bands { cells.push(v.to_cell()); }
        if let Some(v) = &self.optics_perc_extra_bands { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optics_params_default() {
        let params = OpticsParams::default();
        assert!(params.optics_xc_functional.is_none());
        assert!(params.optics_nbands.is_none());
        assert!(params.optics_nextra_bands.is_none());
        assert!(params.optics_perc_extra_bands.is_none());
    }

    #[test]
    fn test_optics_params_builder() {
        let params = OpticsParams::builder().build();
        assert!(params.optics_xc_functional.is_none());
        assert!(params.optics_nbands.is_none());
    }

    #[test]
    fn test_optics_params_to_cell_file_empty() {
        let params = OpticsParams::default();
        let cells = params.to_cell_file();
        assert_eq!(cells.len(), 0);
    }
}
