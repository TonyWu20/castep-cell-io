use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromCellFile, ToCellFile, ToCell, FromKeyValue};
use super::solvation::*;

/// Solvation parameters for CASTEP calculations
///
/// This group contains settings that control implicit solvation models,
/// including boundary conditions, dielectric embedding, and surface tension parameters.
#[derive(Debug, Clone, Default, Builder)]
pub struct SolvationParams {
    pub boundary_type: Option<BoundaryType>,
    pub dielec_emb_func_method: Option<DielecEmbFuncMethod>,
    pub dielec_emb_bulk_permittivity: Option<DielecEmbBulkPermittivity>,
    pub implicit_solvent_apolar_factor: Option<ImplicitSolventApolarFactor>,
    pub implicit_solvent_apolar_term: Option<ImplicitSolventApolarTerm>,
    pub implicit_solvent_surface_tension: Option<ImplicitSolventSurfaceTension>,
    pub use_smeared_ions: Option<UseSmearediIons>,
}

impl SolvationParams {
    /// Validates intra-group constraints for solvation parameters
    pub fn validate(self) -> Result<Self, String> {
        // No specific intra-group validation needed for solvation params
        Ok(self)
    }
}

impl FromCellFile for SolvationParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_boundary_type(BoundaryType::from_cells(tokens).ok().flatten())
            .maybe_dielec_emb_func_method(DielecEmbFuncMethod::from_cells(tokens).ok().flatten())
            .maybe_dielec_emb_bulk_permittivity(DielecEmbBulkPermittivity::from_cells(tokens).ok().flatten())
            .maybe_implicit_solvent_apolar_factor(ImplicitSolventApolarFactor::from_cells(tokens).ok().flatten())
            .maybe_implicit_solvent_apolar_term(ImplicitSolventApolarTerm::from_cells(tokens).ok().flatten())
            .maybe_implicit_solvent_surface_tension(ImplicitSolventSurfaceTension::from_cells(tokens).ok().flatten())
            .maybe_use_smeared_ions(UseSmearediIons::from_cells(tokens).ok().flatten())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for SolvationParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.boundary_type { cells.push(v.to_cell()); }
        if let Some(v) = &self.dielec_emb_func_method { cells.push(v.to_cell()); }
        if let Some(v) = &self.dielec_emb_bulk_permittivity { cells.push(v.to_cell()); }
        if let Some(v) = &self.implicit_solvent_apolar_factor { cells.push(v.to_cell()); }
        if let Some(v) = &self.implicit_solvent_apolar_term { cells.push(v.to_cell()); }
        if let Some(v) = &self.implicit_solvent_surface_tension { cells.push(v.to_cell()); }
        if let Some(v) = &self.use_smeared_ions { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_construction() {
        let params = SolvationParams::default();
        assert!(params.boundary_type.is_none());
        assert!(params.use_smeared_ions.is_none());
    }

    #[test]
    fn test_builder_construction() {
        let params = SolvationParams::builder().build();
        assert!(params.boundary_type.is_none());
    }

    #[test]
    fn test_validate_empty() {
        let params = SolvationParams::default();
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_to_cell_file_empty() {
        let params = SolvationParams::default();
        let cells = params.to_cell_file();
        assert_eq!(cells.len(), 0);
    }
}
