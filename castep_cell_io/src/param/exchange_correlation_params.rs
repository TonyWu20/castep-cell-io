use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromCellFile, ToCellFile, ToCell, FromKeyValue, parse::FromBlock};
use super::exchange_correlation::*;

/// Exchange-correlation parameters for CASTEP calculations
///
/// This group contains settings that control the exchange-correlation functional,
/// including XC functional selection, non-local XC options, and screening parameters.
#[derive(Debug, Clone, Default, Builder)]
pub struct ExchangeCorrelationParams {
    pub k_scrn_averaging_scheme: Option<KScrnAveragingScheme>,
    pub spin_polarized: Option<SpinPolarized>,
    pub xc_functional: Option<XcFunctional>,
    pub nlxc_exchange_reflect_kpts: Option<NlxcExchangeReflectKpts>,
    pub nlxc_impose_trs: Option<NlxcImposeTrs>,
    pub nlxc_ppd_integral: Option<NlxcPpdIntegral>,
    pub nlxc_re_est_k_scrn: Option<NlxcReEstKScrn>,
    pub nlxc_page_ex_pot: Option<NlxcPageExPot>,
    pub nlxc_ppd_size_x: Option<NlxcPpdSizeX>,
    pub nlxc_ppd_size_y: Option<NlxcPpdSizeY>,
    pub nlxc_ppd_size_z: Option<NlxcPpdSizeZ>,
    pub xc_definition: Option<XcDefinition>,
}

impl ExchangeCorrelationParams {
    /// Validates intra-group constraints for exchange-correlation parameters
    pub fn validate(self) -> Result<Self, String> {
        Ok(self)
    }
}

impl FromCellFile for ExchangeCorrelationParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_k_scrn_averaging_scheme(KScrnAveragingScheme::from_cells(tokens).ok().flatten())
            .maybe_spin_polarized(SpinPolarized::from_cells(tokens).ok().flatten())
            .maybe_xc_functional(XcFunctional::from_cells(tokens).ok().flatten())
            .maybe_nlxc_exchange_reflect_kpts(NlxcExchangeReflectKpts::from_cells(tokens).ok().flatten())
            .maybe_nlxc_impose_trs(NlxcImposeTrs::from_cells(tokens).ok().flatten())
            .maybe_nlxc_ppd_integral(NlxcPpdIntegral::from_cells(tokens).ok().flatten())
            .maybe_nlxc_re_est_k_scrn(NlxcReEstKScrn::from_cells(tokens).ok().flatten())
            .maybe_nlxc_page_ex_pot(NlxcPageExPot::from_cells(tokens).ok().flatten())
            .maybe_nlxc_ppd_size_x(NlxcPpdSizeX::from_cells(tokens).ok().flatten())
            .maybe_nlxc_ppd_size_y(NlxcPpdSizeY::from_cells(tokens).ok().flatten())
            .maybe_nlxc_ppd_size_z(NlxcPpdSizeZ::from_cells(tokens).ok().flatten())
            .maybe_xc_definition(XcDefinition::from_cells(tokens).ok())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for ExchangeCorrelationParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.k_scrn_averaging_scheme { cells.push(v.to_cell()); }
        if let Some(v) = &self.spin_polarized { cells.push(v.to_cell()); }
        if let Some(v) = &self.xc_functional { cells.push(v.to_cell()); }
        if let Some(v) = &self.nlxc_exchange_reflect_kpts { cells.push(v.to_cell()); }
        if let Some(v) = &self.nlxc_impose_trs { cells.push(v.to_cell()); }
        if let Some(v) = &self.nlxc_ppd_integral { cells.push(v.to_cell()); }
        if let Some(v) = &self.nlxc_re_est_k_scrn { cells.push(v.to_cell()); }
        if let Some(v) = &self.nlxc_page_ex_pot { cells.push(v.to_cell()); }
        if let Some(v) = &self.nlxc_ppd_size_x { cells.push(v.to_cell()); }
        if let Some(v) = &self.nlxc_ppd_size_y { cells.push(v.to_cell()); }
        if let Some(v) = &self.nlxc_ppd_size_z { cells.push(v.to_cell()); }
        if let Some(v) = &self.xc_definition { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creates_empty_struct() {
        let params = ExchangeCorrelationParams::builder().build();
        assert!(params.xc_functional.is_none());
        assert!(params.spin_polarized.is_none());
    }

    #[test]
    fn test_builder_with_single_field() {
        let params = ExchangeCorrelationParams::builder()
            .maybe_spin_polarized(Some(SpinPolarized(true)))
            .build();
        assert_eq!(params.spin_polarized.unwrap().0, true);
    }

    #[test]
    fn test_to_cell_file_empty() {
        let params = ExchangeCorrelationParams::builder().build();
        let cells = params.to_cell_file();
        assert!(cells.is_empty());
    }

    #[test]
    fn test_to_cell_file_with_fields() {
        let params = ExchangeCorrelationParams::builder()
            .maybe_spin_polarized(Some(SpinPolarized(true)))
            .maybe_k_scrn_averaging_scheme(Some(KScrnAveragingScheme::AveDen))
            .build();
        let cells = params.to_cell_file();
        assert_eq!(cells.len(), 2);
    }

    #[test]
    fn test_validate_succeeds() {
        let params = ExchangeCorrelationParams::builder().build();
        assert!(params.validate().is_ok());
    }
}
