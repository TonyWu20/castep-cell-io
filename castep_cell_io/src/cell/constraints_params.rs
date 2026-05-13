use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromBlock, FromCellFile, ToCellFile, ToCell, FromKeyValue};

use super::constraints::*;

/// Movement constraints for ions and cell
///
/// [`CellConstraints`] supersedes [`FixAllCell`] — if both are present,
/// `fix_all_cell` is silently dropped (following CASTEP's own behavior).
#[derive(Debug, Clone, Default, Builder)]
pub struct ConstraintsParams {
    pub fix_com: Option<FixCOM>,
    pub ionic_constraints: Option<IonicConstraints>,
    pub nonlinear_constraints: Option<NonlinearConstraints>,
    pub fix_all_ions: Option<FixAllIons>,
    pub fix_all_cell: Option<FixAllCell>,
    pub cell_constraints: Option<CellConstraints>,
    pub fix_vol: Option<FixVOL>,
}

impl ConstraintsParams {
    /// Validates constraints and applies superseding rules.
    ///
    /// If `cell_constraints` is present, `fix_all_cell` is set to `None`
    /// (following CASTEP's behavior where CELL_CONSTRAINTS supersedes FIX_ALL_CELL).
    pub fn validate(mut self) -> Result<Self, String> {
        if self.cell_constraints.is_some() && self.fix_all_cell.is_some() {
            eprintln!(
                "Warning: CELL_CONSTRAINTS supersedes FIX_ALL_CELL. FIX_ALL_CELL will be ignored."
            );
            self.fix_all_cell = None;
        }
        Ok(self)
    }
}

impl FromCellFile for ConstraintsParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_fix_com(FixCOM::from_cells(tokens).ok().flatten())
            .maybe_ionic_constraints(IonicConstraints::from_cells(tokens).ok())
            .maybe_nonlinear_constraints(NonlinearConstraints::from_cells(tokens).ok())
            .maybe_fix_all_ions(FixAllIons::from_cells(tokens).ok().flatten())
            .maybe_fix_all_cell(FixAllCell::from_cells(tokens).ok().flatten())
            .maybe_cell_constraints(CellConstraints::from_cells(tokens).ok())
            .maybe_fix_vol(FixVOL::from_cells(tokens).ok().flatten())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for ConstraintsParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.fix_com { cells.push(v.to_cell()); }
        if let Some(v) = &self.ionic_constraints { cells.push(v.to_cell()); }
        if let Some(v) = &self.nonlinear_constraints { cells.push(v.to_cell()); }
        if let Some(v) = &self.fix_all_ions { cells.push(v.to_cell()); }
        if let Some(v) = &self.fix_all_cell { cells.push(v.to_cell()); }
        if let Some(v) = &self.cell_constraints { cells.push(v.to_cell()); }
        if let Some(v) = &self.fix_vol { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_default() {
        let p = ConstraintsParams::default();
        let r = p.validate().unwrap();
        assert!(r.fix_com.is_none());
    }

    #[test]
    fn test_cell_constraints_supersedes_fix_all_cell() {
        let p = ConstraintsParams {
            cell_constraints: Some(CellConstraints {
                lengths: [1, 1, 1],
                angles: [0, 0, 0],
            }),
            fix_all_cell: Some(FixAllCell(true)),
            ..Default::default()
        };
        let r = p.validate().unwrap();
        // cell_constraints should still be present
        assert!(r.cell_constraints.is_some());
        // fix_all_cell should be cleared
        assert!(r.fix_all_cell.is_none());
    }

    #[test]
    fn test_no_superseding_when_cell_constraints_absent() {
        let p = ConstraintsParams {
            fix_all_cell: Some(FixAllCell(true)),
            ..Default::default()
        };
        let r = p.validate().unwrap();
        assert!(r.fix_all_cell.is_some());
        assert_eq!(r.fix_all_cell.unwrap().0, true);
    }
}
