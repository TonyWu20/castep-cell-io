use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromBlock, FromCellFile, ToCellFile, ToCell, FromKeyValue};

use super::phonon::*;

/// Phonon fine k-point parameters
///
/// Contains fine k-point settings for phonon calculations. Only one of
/// [`PhononFineKpointPath`], [`PhononFineKpointList`], [`PhononFineKpointsMpGrid`],
/// or [`PhononFineKpointsMpSpacing`] may be present.
///
/// [`PhononFineKpointsMpOffset`] is an optional companion to MP grid/spacing.
#[derive(Debug, Clone, Default, Builder)]
pub struct PhononFineParams {
    pub phonon_fine_kpoint_list: Option<PhononFineKpointList>,
    pub phonon_fine_kpoint_path: Option<PhononFineKpointPath>,
    pub phonon_fine_kpoint_path_spacing: Option<PhononFineKpointPathSpacing>,
    pub phonon_fine_kpoints_mp_grid: Option<PhononFineKpointsMpGrid>,
    pub phonon_fine_kpoints_mp_spacing: Option<PhononFineKpointsMpSpacing>,
    pub phonon_fine_kpoints_mp_offset: Option<PhononFineKpointsMpOffset>,
}

impl PhononFineParams {
    /// Validates phonon fine k-point mutual exclusion.
    ///
    /// At most one of {path, list, mp_grid, mp_spacing} may be present.
    pub fn validate(self) -> Result<Self, String> {
        let count = [
            self.phonon_fine_kpoint_path.is_some(),
            self.phonon_fine_kpoint_list.is_some(),
            self.phonon_fine_kpoints_mp_grid.is_some(),
            self.phonon_fine_kpoints_mp_spacing.is_some(),
        ]
        .into_iter()
        .filter(|&b| b)
        .count();

        if count > 1 {
            return Err("Only one of PHONON_FINE_KPOINT_PATH, PHONON_FINE_KPOINT_LIST, PHONON_FINE_KPOINTS_MP_GRID, or PHONON_FINE_KPOINTS_MP_SPACING may be present".into());
        }
        Ok(self)
    }
}

impl FromCellFile for PhononFineParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_phonon_fine_kpoint_list(PhononFineKpointList::from_cells(tokens).ok())
            .maybe_phonon_fine_kpoint_path(PhononFineKpointPath::from_cells(tokens).ok())
            .maybe_phonon_fine_kpoint_path_spacing(PhononFineKpointPathSpacing::from_cells(tokens).ok().flatten())
            .maybe_phonon_fine_kpoints_mp_grid(PhononFineKpointsMpGrid::from_cells(tokens).ok().flatten())
            .maybe_phonon_fine_kpoints_mp_spacing(PhononFineKpointsMpSpacing::from_cells(tokens).ok().flatten())
            .maybe_phonon_fine_kpoints_mp_offset(PhononFineKpointsMpOffset::from_cells(tokens).ok().flatten())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for PhononFineParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.phonon_fine_kpoint_list { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_fine_kpoint_path { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_fine_kpoint_path_spacing { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_fine_kpoints_mp_grid { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_fine_kpoints_mp_spacing { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_fine_kpoints_mp_offset { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_default() {
        let p = PhononFineParams::default();
        assert!(p.validate().is_ok());
    }

    #[test]
    fn test_validate_single_method_ok() {
        let p = PhononFineParams {
            phonon_fine_kpoint_path: Some(PhononFineKpointPath::builder().build()),
            ..Default::default()
        };
        assert!(p.validate().is_ok());
    }

    #[test]
    fn test_validate_path_and_list_err() {
        let p = PhononFineParams {
            phonon_fine_kpoint_path: Some(PhononFineKpointPath::builder().build()),
            phonon_fine_kpoint_list: Some(PhononFineKpointList::builder().build()),
            ..Default::default()
        };
        assert!(p.validate().is_err());
    }

    #[test]
    fn test_validate_path_and_mp_grid_err() {
        let p = PhononFineParams {
            phonon_fine_kpoint_path: Some(PhononFineKpointPath::builder().build()),
            phonon_fine_kpoints_mp_grid: Some(PhononFineKpointsMpGrid([2, 2, 2])),
            ..Default::default()
        };
        assert!(p.validate().is_err());
    }

    #[test]
    fn test_validate_mp_grid_with_offset_ok() {
        let p = PhononFineParams {
            phonon_fine_kpoints_mp_grid: Some(PhononFineKpointsMpGrid([2, 2, 2])),
            phonon_fine_kpoints_mp_offset: Some(PhononFineKpointsMpOffset([0.0, 0.0, 0.0])),
            ..Default::default()
        };
        assert!(p.validate().is_ok());
    }
}
