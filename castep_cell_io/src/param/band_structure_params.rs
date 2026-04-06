use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromCellFile, ToCellFile, ToCell, FromKeyValue};
use super::band_structure::*;

/// Band structure calculation parameters for CASTEP calculations
///
/// This group contains settings that control band structure calculations,
/// including convergence criteria, number of bands, and exchange-correlation functional.
///
/// # Validation
/// Only one of `bs_nbands`, `bs_nextra_bands`, or `bs_perc_extra_bands` may be specified.
#[derive(Debug, Clone, Default, Builder)]
pub struct BandStructureParams {
    pub bs_eigenvalue_tol: Option<BsEigenvalueTol>,
    pub bs_max_cg_steps: Option<BsMaxCgSteps>,
    pub bs_max_iter: Option<BsMaxIter>,
    pub bs_nbands: Option<BsNbands>,
    pub bs_nextra_bands: Option<BsNextraBands>,
    pub bs_perc_extra_bands: Option<BsPercExtraBands>,
    pub bs_re_est_k_scrn: Option<BsReEstKScrn>,
    pub bs_xc_functional: Option<BsXcFunctional>,
}

impl BandStructureParams {
    /// Validates intra-group constraints for band structure parameters
    ///
    /// # Errors
    /// Returns an error if more than one of `bs_nbands`, `bs_nextra_bands`, or `bs_perc_extra_bands` is specified.
    pub fn validate(self) -> Result<Self, String> {
        // Check mutual exclusivity of bs_nbands, bs_nextra_bands, and bs_perc_extra_bands
        let count = [
            self.bs_nbands.is_some(),
            self.bs_nextra_bands.is_some(),
            self.bs_perc_extra_bands.is_some(),
        ].iter().filter(|&&x| x).count();

        if count > 1 {
            return Err("BS_NBANDS, BS_NEXTRA_BANDS, and BS_PERC_EXTRA_BANDS are mutually exclusive. Only one may be specified.".into());
        }

        Ok(self)
    }
}

impl FromCellFile for BandStructureParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_bs_eigenvalue_tol(BsEigenvalueTol::from_cells(tokens).ok().flatten())
            .maybe_bs_max_cg_steps(BsMaxCgSteps::from_cells(tokens).ok().flatten())
            .maybe_bs_max_iter(BsMaxIter::from_cells(tokens).ok().flatten())
            .maybe_bs_nbands(BsNbands::from_cells(tokens).ok().flatten())
            .maybe_bs_nextra_bands(BsNextraBands::from_cells(tokens).ok().flatten())
            .maybe_bs_perc_extra_bands(BsPercExtraBands::from_cells(tokens).ok().flatten())
            .maybe_bs_re_est_k_scrn(BsReEstKScrn::from_cells(tokens).ok().flatten())
            .maybe_bs_xc_functional(BsXcFunctional::from_cells(tokens).ok().flatten())
            .build()
            .validate()
            .map_err(Error::Message)
    }
}

impl ToCellFile for BandStructureParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.bs_eigenvalue_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.bs_max_cg_steps { cells.push(v.to_cell()); }
        if let Some(v) = &self.bs_max_iter { cells.push(v.to_cell()); }
        if let Some(v) = &self.bs_nbands { cells.push(v.to_cell()); }
        if let Some(v) = &self.bs_nextra_bands { cells.push(v.to_cell()); }
        if let Some(v) = &self.bs_perc_extra_bands { cells.push(v.to_cell()); }
        if let Some(v) = &self.bs_re_est_k_scrn { cells.push(v.to_cell()); }
        if let Some(v) = &self.bs_xc_functional { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_band_structure_params_default() {
        let params = BandStructureParams::default();
        assert!(params.bs_nbands.is_none());
        assert!(params.bs_nextra_bands.is_none());
        assert!(params.bs_perc_extra_bands.is_none());
    }

    #[test]
    fn test_band_structure_params_builder() {
        let params = BandStructureParams::builder()
            .maybe_bs_max_iter(Some(BsMaxIter(100)))
            .build();
        assert_eq!(params.bs_max_iter.unwrap().0, 100);
    }

    #[test]
    fn test_band_structure_params_validate_ok() {
        let params = BandStructureParams::default();
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_band_structure_params_validate_single_nbands() {
        let params = BandStructureParams::builder()
            .maybe_bs_nbands(Some(BsNbands(128)))
            .build();
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_band_structure_params_validate_single_nextra_bands() {
        let params = BandStructureParams::builder()
            .maybe_bs_nextra_bands(Some(BsNextraBands(10)))
            .build();
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_band_structure_params_validate_single_perc_extra_bands() {
        let params = BandStructureParams::builder()
            .maybe_bs_perc_extra_bands(Some(BsPercExtraBands(10.0)))
            .build();
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_band_structure_params_validate_nbands_and_nextra_bands_conflict() {
        let params = BandStructureParams::builder()
            .maybe_bs_nbands(Some(BsNbands(128)))
            .maybe_bs_nextra_bands(Some(BsNextraBands(10)))
            .build();
        let result = params.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("mutually exclusive"));
    }

    #[test]
    fn test_band_structure_params_validate_nbands_and_perc_extra_bands_conflict() {
        let params = BandStructureParams::builder()
            .maybe_bs_nbands(Some(BsNbands(128)))
            .maybe_bs_perc_extra_bands(Some(BsPercExtraBands(10.0)))
            .build();
        let result = params.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("mutually exclusive"));
    }

    #[test]
    fn test_band_structure_params_validate_nextra_bands_and_perc_extra_bands_conflict() {
        let params = BandStructureParams::builder()
            .maybe_bs_nextra_bands(Some(BsNextraBands(10)))
            .maybe_bs_perc_extra_bands(Some(BsPercExtraBands(10.0)))
            .build();
        let result = params.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("mutually exclusive"));
    }

    #[test]
    fn test_band_structure_params_validate_all_three_conflict() {
        let params = BandStructureParams::builder()
            .maybe_bs_nbands(Some(BsNbands(128)))
            .maybe_bs_nextra_bands(Some(BsNextraBands(10)))
            .maybe_bs_perc_extra_bands(Some(BsPercExtraBands(10.0)))
            .build();
        let result = params.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("mutually exclusive"));
    }

    #[test]
    fn test_band_structure_params_to_cell_file() {
        let params = BandStructureParams::builder()
            .maybe_bs_max_iter(Some(BsMaxIter(100)))
            .maybe_bs_nbands(Some(BsNbands(128)))
            .build();
        let cells = params.to_cell_file();
        assert_eq!(cells.len(), 2);
    }
}
