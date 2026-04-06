use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromCellFile, ToCellFile, ToCell, FromKeyValue};
use super::basis_set::*;

/// Basis set parameters for CASTEP calculations
///
/// This group contains settings that control the basis set and grid,
/// including cutoff energy, grid scaling, and finite basis corrections.
#[derive(Debug, Clone, Default, Builder)]
pub struct BasisSetParams {
    pub basis_de_dloge: Option<BasisDeDloge>,
    pub basis_precision: Option<BasisPrecision>,
    pub cutoff_energy: Option<CutOffEnergy>,
    pub fine_gmax: Option<FineGmax>,
    pub fine_grid_scale: Option<FineGridScale>,
    pub finite_basis_corr: Option<FiniteBasisCorr>,
    pub finite_basis_npoints: Option<FiniteBasisNpoints>,
    pub finite_basis_spacing: Option<FiniteBasisSpacing>,
    pub fixed_npw: Option<FixedNpw>,
    pub grid_scale: Option<GridScale>,
}

impl BasisSetParams {
    /// Validates intra-group constraints for basis set parameters
    pub fn validate(self) -> Result<Self, String> {
        // No specific intra-group validation needed for basis set params
        Ok(self)
    }
}

impl FromCellFile for BasisSetParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_basis_de_dloge(BasisDeDloge::from_cells(tokens).ok().flatten())
            .maybe_basis_precision(BasisPrecision::from_cells(tokens).ok().flatten())
            .maybe_cutoff_energy(CutOffEnergy::from_cells(tokens).ok().flatten())
            .maybe_fine_gmax(FineGmax::from_cells(tokens).ok().flatten())
            .maybe_fine_grid_scale(FineGridScale::from_cells(tokens).ok().flatten())
            .maybe_finite_basis_corr(FiniteBasisCorr::from_cells(tokens).ok().flatten())
            .maybe_finite_basis_npoints(FiniteBasisNpoints::from_cells(tokens).ok().flatten())
            .maybe_finite_basis_spacing(FiniteBasisSpacing::from_cells(tokens).ok().flatten())
            .maybe_fixed_npw(FixedNpw::from_cells(tokens).ok().flatten())
            .maybe_grid_scale(GridScale::from_cells(tokens).ok().flatten())
            .build()
            .validate()
            .map_err(|e| Error::Message(e))
    }
}

impl ToCellFile for BasisSetParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.basis_de_dloge { cells.push(v.to_cell()); }
        if let Some(v) = &self.basis_precision { cells.push(v.to_cell()); }
        if let Some(v) = &self.cutoff_energy { cells.push(v.to_cell()); }
        if let Some(v) = &self.fine_gmax { cells.push(v.to_cell()); }
        if let Some(v) = &self.fine_grid_scale { cells.push(v.to_cell()); }
        if let Some(v) = &self.finite_basis_corr { cells.push(v.to_cell()); }
        if let Some(v) = &self.finite_basis_npoints { cells.push(v.to_cell()); }
        if let Some(v) = &self.finite_basis_spacing { cells.push(v.to_cell()); }
        if let Some(v) = &self.fixed_npw { cells.push(v.to_cell()); }
        if let Some(v) = &self.grid_scale { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basis_set_params_default() {
        let params = BasisSetParams::default();
        assert!(params.cutoff_energy.is_none());
        assert!(params.grid_scale.is_none());
    }

    #[test]
    fn test_basis_set_params_builder() {
        let params = BasisSetParams::builder()
            .maybe_cutoff_energy(Some(CutOffEnergy { value: 900.0, unit: None }))
            .build();
        assert_eq!(params.cutoff_energy.unwrap().value, 900.0);
    }

    #[test]
    fn test_basis_set_params_validate() {
        let params = BasisSetParams::default();
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_basis_set_params_to_cell_file() {
        let params = BasisSetParams::builder()
            .maybe_cutoff_energy(Some(CutOffEnergy { value: 900.0, unit: None }))
            .build();
        let cells = params.to_cell_file();
        assert_eq!(cells.len(), 1);
    }
}
