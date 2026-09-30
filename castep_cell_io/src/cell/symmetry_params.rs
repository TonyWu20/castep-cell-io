use bon::Builder;
use castep_cell_fmt::query::has_flag;
use castep_cell_fmt::{Cell, CResult, Error, FromBlock, FromCellFile, ToCellFile, ToCell, FromKeyValue};

use super::symmetry::*;

/// Symmetry parameters
///
/// At most one of [`SymmetryOps`] or [`SymmetryGenerate`] may be present.
/// [`SymmetryTol`] and [`SnapToSymmetry`] are independent and can coexist
/// with either.
#[derive(Debug, Clone, Default, Builder)]
pub struct SymmetryParams {
    pub symmetry_ops: Option<SymmetryOps>,
    pub symmetry_generate: Option<SymmetryGenerate>,
    pub symmetry_tol: Option<SymmetryTol>,
    /// 6.11: when present, forces the lattice and ionic positions to obey
    /// the supplied/generated symmetries. Independent of the other fields.
    pub snap_to_symmetry: Option<SnapToSymmetry>,
}

impl SymmetryParams {
    pub fn validate(self) -> Result<Self, String> {
        if self.symmetry_ops.is_some() && self.symmetry_generate.is_some() {
            return Err("Only one of SYMMETRY_OPS and SYMMETRY_GENERATE may be present".into());
        }
        Ok(self)
    }
}

impl FromCellFile for SymmetryParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        let symmetry_generate = has_flag(tokens, "SYMMETRY_GENERATE").then_some(SymmetryGenerate);
        let snap_to_symmetry = has_flag(tokens, "SNAP_TO_SYMMETRY").then_some(SnapToSymmetry);
        Self::builder()
            .maybe_symmetry_ops(SymmetryOps::from_cells(tokens).ok())
            .maybe_symmetry_generate(symmetry_generate)
            .maybe_symmetry_tol(SymmetryTol::from_cells(tokens).ok().flatten())
            .maybe_snap_to_symmetry(snap_to_symmetry)
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for SymmetryParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.symmetry_ops { cells.push(v.to_cell()); }
        if let Some(v) = &self.symmetry_generate { cells.push(v.to_cell()); }
        if let Some(v) = &self.symmetry_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.snap_to_symmetry { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::LengthUnit;

    #[test]
    fn test_validate_ops_only_ok() {
        let p = SymmetryParams {
            symmetry_ops: Some(SymmetryOps { ops: vec![] }),
            ..Default::default()
        };
        assert!(p.validate().is_ok());
    }

    #[test]
    fn test_validate_ops_and_generate_err() {
        let p = SymmetryParams {
            symmetry_ops: Some(SymmetryOps { ops: vec![] }),
            symmetry_generate: Some(SymmetryGenerate),
            ..Default::default()
        };
        assert!(p.validate().is_err());
    }

    #[test]
    fn test_validate_generate_with_tol_ok() {
        let p = SymmetryParams {
            symmetry_generate: Some(SymmetryGenerate),
            symmetry_tol: Some(SymmetryTol::builder().value(0.01).unit(LengthUnit::Ang).build()),
            ..Default::default()
        };
        assert!(p.validate().is_ok());
    }
}
