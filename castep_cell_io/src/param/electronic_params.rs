use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromCellFile, ToCellFile, ToCell, FromKeyValue};
use super::electronic::*;

/// Electronic structure parameters for CASTEP calculations
///
/// This group contains settings that control electronic structure calculations,
/// including charge, spin, band structure, and semi-empirical dispersion corrections (SEDC).
#[derive(Debug, Clone, Default, Builder)]
pub struct ElectronicParams {
    pub charge: Option<Charge>,
    pub nbands: Option<Nbands>,
    pub ndown: Option<Ndown>,
    pub nelectrons: Option<Nelectrons>,
    pub nextra_bands: Option<NextraBands>,
    pub nup: Option<Nup>,
    pub perc_extra_bands: Option<PercExtraBands>,
    pub sedc_apply: Option<SedcApply>,
    pub sedc_d_g06: Option<SedcDG06>,
    pub sedc_d_jchs: Option<SedcDJchs>,
    pub sedc_d_ts: Option<SedcDTs>,
    pub sedc_lambda_obs: Option<SedcLambdaObs>,
    pub sedc_n_obs: Option<SedcNObs>,
    pub sedc_s6_g06: Option<SedcS6G06>,
    pub sedc_s6_jchs: Option<SedcS6Jchs>,
    pub sedc_scheme: Option<SedcScheme>,
    pub sedc_sr_jchs: Option<SedcSrJchs>,
    pub sedc_sr_ts: Option<SedcSrTs>,
    pub spin: Option<Spin>,
}

impl ElectronicParams {
    /// Validates intra-group constraints for electronic parameters
    pub fn validate(self) -> Result<Self, String> {
        // Check mutual exclusivity of nbands, nextra_bands, and perc_extra_bands
        let count = [
            self.nbands.is_some(),
            self.nextra_bands.is_some(),
            self.perc_extra_bands.is_some(),
        ].iter().filter(|&&x| x).count();

        if count > 1 {
            return Err("NBANDS, NEXTRA_BANDS, and PERC_EXTRA_BANDS are mutually exclusive. Only one may be specified.".into());
        }

        Ok(self)
    }
}

impl FromCellFile for ElectronicParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_charge(Charge::from_cells(tokens).ok().flatten())
            .maybe_nbands(Nbands::from_cells(tokens).ok().flatten())
            .maybe_ndown(Ndown::from_cells(tokens).ok().flatten())
            .maybe_nelectrons(Nelectrons::from_cells(tokens).ok().flatten())
            .maybe_nextra_bands(NextraBands::from_cells(tokens).ok().flatten())
            .maybe_nup(Nup::from_cells(tokens).ok().flatten())
            .maybe_perc_extra_bands(PercExtraBands::from_cells(tokens).ok().flatten())
            .maybe_sedc_apply(SedcApply::from_cells(tokens).ok().flatten())
            .maybe_sedc_d_g06(SedcDG06::from_cells(tokens).ok().flatten())
            .maybe_sedc_d_jchs(SedcDJchs::from_cells(tokens).ok().flatten())
            .maybe_sedc_d_ts(SedcDTs::from_cells(tokens).ok().flatten())
            .maybe_sedc_lambda_obs(SedcLambdaObs::from_cells(tokens).ok().flatten())
            .maybe_sedc_n_obs(SedcNObs::from_cells(tokens).ok().flatten())
            .maybe_sedc_s6_g06(SedcS6G06::from_cells(tokens).ok().flatten())
            .maybe_sedc_s6_jchs(SedcS6Jchs::from_cells(tokens).ok().flatten())
            .maybe_sedc_scheme(SedcScheme::from_cells(tokens).ok().flatten())
            .maybe_sedc_sr_jchs(SedcSrJchs::from_cells(tokens).ok().flatten())
            .maybe_sedc_sr_ts(SedcSrTs::from_cells(tokens).ok().flatten())
            .maybe_spin(Spin::from_cells(tokens).ok().flatten())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for ElectronicParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.charge { cells.push(v.to_cell()); }
        if let Some(v) = &self.nbands { cells.push(v.to_cell()); }
        if let Some(v) = &self.ndown { cells.push(v.to_cell()); }
        if let Some(v) = &self.nelectrons { cells.push(v.to_cell()); }
        if let Some(v) = &self.nextra_bands { cells.push(v.to_cell()); }
        if let Some(v) = &self.nup { cells.push(v.to_cell()); }
        if let Some(v) = &self.perc_extra_bands { cells.push(v.to_cell()); }
        if let Some(v) = &self.sedc_apply { cells.push(v.to_cell()); }
        if let Some(v) = &self.sedc_d_g06 { cells.push(v.to_cell()); }
        if let Some(v) = &self.sedc_d_jchs { cells.push(v.to_cell()); }
        if let Some(v) = &self.sedc_d_ts { cells.push(v.to_cell()); }
        if let Some(v) = &self.sedc_lambda_obs { cells.push(v.to_cell()); }
        if let Some(v) = &self.sedc_n_obs { cells.push(v.to_cell()); }
        if let Some(v) = &self.sedc_s6_g06 { cells.push(v.to_cell()); }
        if let Some(v) = &self.sedc_s6_jchs { cells.push(v.to_cell()); }
        if let Some(v) = &self.sedc_scheme { cells.push(v.to_cell()); }
        if let Some(v) = &self.sedc_sr_jchs { cells.push(v.to_cell()); }
        if let Some(v) = &self.sedc_sr_ts { cells.push(v.to_cell()); }
        if let Some(v) = &self.spin { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_builder() {
        let params = ElectronicParams::builder().build();
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_builder_with_single_field() {
        let params = ElectronicParams::builder()
            .maybe_charge(Some(Charge(1.0)))
            .build();
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_nbands_only() {
        let params = ElectronicParams::builder()
            .maybe_nbands(Some(Nbands(100)))
            .build();
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_nextra_bands_only() {
        let params = ElectronicParams::builder()
            .maybe_nextra_bands(Some(NextraBands(10)))
            .build();
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_perc_extra_bands_only() {
        let params = ElectronicParams::builder()
            .maybe_perc_extra_bands(Some(PercExtraBands(10.0)))
            .build();
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_nbands_and_nextra_bands_mutual_exclusivity() {
        let params = ElectronicParams::builder()
            .maybe_nbands(Some(Nbands(100)))
            .maybe_nextra_bands(Some(NextraBands(10)))
            .build();
        assert!(params.validate().is_err());
    }

    #[test]
    fn test_nbands_and_perc_extra_bands_mutual_exclusivity() {
        let params = ElectronicParams::builder()
            .maybe_nbands(Some(Nbands(100)))
            .maybe_perc_extra_bands(Some(PercExtraBands(10.0)))
            .build();
        assert!(params.validate().is_err());
    }

    #[test]
    fn test_nextra_bands_and_perc_extra_bands_mutual_exclusivity() {
        let params = ElectronicParams::builder()
            .maybe_nextra_bands(Some(NextraBands(10)))
            .maybe_perc_extra_bands(Some(PercExtraBands(10.0)))
            .build();
        assert!(params.validate().is_err());
    }

    #[test]
    fn test_all_three_mutual_exclusivity() {
        let params = ElectronicParams::builder()
            .maybe_nbands(Some(Nbands(100)))
            .maybe_nextra_bands(Some(NextraBands(10)))
            .maybe_perc_extra_bands(Some(PercExtraBands(10.0)))
            .build();
        assert!(params.validate().is_err());
    }

    #[test]
    fn test_to_cell_file_empty() {
        let params = ElectronicParams::builder().build();
        let cells = params.to_cell_file();
        assert_eq!(cells.len(), 0);
    }

    #[test]
    fn test_to_cell_file_with_fields() {
        let params = ElectronicParams::builder()
            .maybe_charge(Some(Charge(1.0)))
            .maybe_nbands(Some(Nbands(100)))
            .maybe_spin(Some(Spin(2.0)))
            .build();
        let cells = params.to_cell_file();
        assert_eq!(cells.len(), 3);
    }

    #[test]
    fn test_builder_with_multiple_sedc_fields() {
        let params = ElectronicParams::builder()
            .maybe_charge(Some(Charge(1.0)))
            .maybe_nelectrons(Some(Nelectrons(32.0)))
            .build();
        assert_eq!(params.charge, Some(Charge(1.0)));
        assert!(params.validate().is_ok());
    }
}
