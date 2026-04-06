use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromCellFile, ToCellFile, ToCell, FromKeyValue};
use super::geometry_optimization::*;

/// Geometry optimization parameters for CASTEP calculations
///
/// This group contains settings that control geometry optimization runs,
/// including convergence criteria, optimization method, and related parameters.
#[derive(Debug, Clone, Default, Builder)]
pub struct GeometryOptimizationParams {
    pub geom_convergence_win: Option<GeomConvergenceWin>,
    pub geom_disp_tol: Option<GeomDispTol>,
    pub geom_energy_tol: Option<GeomEnergyTol>,
    pub geom_force_tol: Option<GeomForceTol>,
    pub geom_frequency_est: Option<GeomFrequencyEst>,
    pub geom_max_iter: Option<GeomMaxIter>,
    pub geom_method: Option<GeomMethod>,
    pub geom_modulus_est: Option<GeomModulusEst>,
    pub geom_preconditioner: Option<GeomPreconditioner>,
    pub geom_spin_fix: Option<GeomSpinFix>,
    pub geom_stress_tol: Option<GeomStressTol>,
}

impl GeometryOptimizationParams {
    /// Validates intra-group constraints for geometry optimization parameters
    pub fn validate(self) -> Result<Self, String> {
        // Currently no mutual exclusivity constraints for geometry optimization
        Ok(self)
    }
}

impl FromCellFile for GeometryOptimizationParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_geom_convergence_win(GeomConvergenceWin::from_cells(tokens).ok().flatten())
            .maybe_geom_disp_tol(GeomDispTol::from_cells(tokens).ok().flatten())
            .maybe_geom_energy_tol(GeomEnergyTol::from_cells(tokens).ok().flatten())
            .maybe_geom_force_tol(GeomForceTol::from_cells(tokens).ok().flatten())
            .maybe_geom_frequency_est(GeomFrequencyEst::from_cells(tokens).ok().flatten())
            .maybe_geom_max_iter(GeomMaxIter::from_cells(tokens).ok().flatten())
            .maybe_geom_method(GeomMethod::from_cells(tokens).ok().flatten())
            .maybe_geom_modulus_est(GeomModulusEst::from_cells(tokens).ok().flatten())
            .maybe_geom_preconditioner(GeomPreconditioner::from_cells(tokens).ok().flatten())
            .maybe_geom_spin_fix(GeomSpinFix::from_cells(tokens).ok().flatten())
            .maybe_geom_stress_tol(GeomStressTol::from_cells(tokens).ok().flatten())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for GeometryOptimizationParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.geom_convergence_win { cells.push(v.to_cell()); }
        if let Some(v) = &self.geom_disp_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.geom_energy_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.geom_force_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.geom_frequency_est { cells.push(v.to_cell()); }
        if let Some(v) = &self.geom_max_iter { cells.push(v.to_cell()); }
        if let Some(v) = &self.geom_method { cells.push(v.to_cell()); }
        if let Some(v) = &self.geom_modulus_est { cells.push(v.to_cell()); }
        if let Some(v) = &self.geom_preconditioner { cells.push(v.to_cell()); }
        if let Some(v) = &self.geom_spin_fix { cells.push(v.to_cell()); }
        if let Some(v) = &self.geom_stress_tol { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_builder() {
        let params = GeometryOptimizationParams::builder().build();
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_builder_with_single_field() {
        let params = GeometryOptimizationParams::builder()
            .maybe_geom_max_iter(Some(GeomMaxIter(100)))
            .build();
        assert_eq!(params.geom_max_iter, Some(GeomMaxIter(100)));
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_builder_with_multiple_fields() {
        let params = GeometryOptimizationParams::builder()
            .maybe_geom_max_iter(Some(GeomMaxIter(100)))
            .maybe_geom_method(Some(GeomMethod::Bfgs))
            .build();
        assert_eq!(params.geom_max_iter, Some(GeomMaxIter(100)));
        assert_eq!(params.geom_method, Some(GeomMethod::Bfgs));
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_to_cell_file_empty() {
        let params = GeometryOptimizationParams::builder().build();
        let cells = params.to_cell_file();
        assert_eq!(cells.len(), 0);
    }

    #[test]
    fn test_to_cell_file_with_fields() {
        let params = GeometryOptimizationParams::builder()
            .maybe_geom_max_iter(Some(GeomMaxIter(100)))
            .maybe_geom_method(Some(GeomMethod::Bfgs))
            .build();
        let cells = params.to_cell_file();
        assert_eq!(cells.len(), 2);
    }

    #[test]
    fn test_validate_always_succeeds() {
        let params = GeometryOptimizationParams::builder()
            .maybe_geom_max_iter(Some(GeomMaxIter(100)))
            .maybe_geom_method(Some(GeomMethod::Bfgs))
            .maybe_geom_force_tol(Some(GeomForceTol { value: 0.01, unit: None }))
            .build();
        assert!(params.validate().is_ok());
    }
}
