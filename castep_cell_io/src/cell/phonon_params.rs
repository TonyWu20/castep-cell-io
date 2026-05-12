use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromBlock, FromCellFile, ToCellFile, ToCell, FromKeyValue};

use super::phonon::*;

/// Phonon (coarse) parameters
///
/// Contains k-point settings for phonon calculations. Only one of
/// [`PhononKpointPath`], [`PhononKpointList`], [`PhononKpointsMpGrid`],
/// or [`PhononKpointsMpSpacing`] may be present.
///
/// [`PhononKpointsMpOffset`] is an optional companion to MP grid/spacing.
/// [`PhononGammaDirections`], [`PhononSupercellMatrix`], and [`SupercellKpointListCastep`]
/// are independent.
#[derive(Debug, Clone, Default, Builder)]
pub struct PhononParams {
    pub phonon_kpoint_list: Option<PhononKpointList>,
    pub phonon_kpoint_path: Option<PhononKpointPath>,
    pub phonon_kpoint_path_spacing: Option<PhononKpointPathSpacing>,
    pub phonon_kpoints_mp_grid: Option<PhononKpointsMpGrid>,
    pub phonon_kpoints_mp_spacing: Option<PhononKpointsMpSpacing>,
    pub phonon_kpoints_mp_offset: Option<PhononKpointsMpOffset>,
    pub phonon_gamma_directions: Option<PhononGammaDirections>,
    pub phonon_supercell_matrix: Option<PhononSupercellMatrix>,
    pub supercell_kpoint_list: Option<SupercellKpointListCastep>,
}

impl PhononParams {
    /// Validates phonon k-point mutual exclusion.
    ///
    /// At most one of {path, list, mp_grid, mp_spacing} may be present.
    pub fn validate(self) -> Result<Self, String> {
        let count = [
            self.phonon_kpoint_path.is_some(),
            self.phonon_kpoint_list.is_some(),
            self.phonon_kpoints_mp_grid.is_some(),
            self.phonon_kpoints_mp_spacing.is_some(),
        ]
        .into_iter()
        .filter(|&b| b)
        .count();

        if count > 1 {
            return Err("Only one of PHONON_KPOINT_PATH, PHONON_KPOINT_LIST, PHONON_KPOINTS_MP_GRID, or PHONON_KPOINTS_MP_SPACING may be present".into());
        }
        Ok(self)
    }
}

impl FromCellFile for PhononParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_phonon_kpoint_list(PhononKpointList::from_cells(tokens).ok())
            .maybe_phonon_kpoint_path(PhononKpointPath::from_cells(tokens).ok())
            .maybe_phonon_kpoint_path_spacing(PhononKpointPathSpacing::from_cells(tokens).ok().flatten())
            .maybe_phonon_kpoints_mp_grid(PhononKpointsMpGrid::from_cells(tokens).ok().flatten())
            .maybe_phonon_kpoints_mp_spacing(PhononKpointsMpSpacing::from_cells(tokens).ok().flatten())
            .maybe_phonon_kpoints_mp_offset(PhononKpointsMpOffset::from_cells(tokens).ok().flatten())
            .maybe_phonon_gamma_directions(PhononGammaDirections::from_cells(tokens).ok())
            .maybe_phonon_supercell_matrix(PhononSupercellMatrix::from_cells(tokens).ok())
            .maybe_supercell_kpoint_list(SupercellKpointListCastep::from_cells(tokens).ok())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for PhononParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.phonon_kpoint_list { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_kpoint_path { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_kpoint_path_spacing { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_kpoints_mp_grid { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_kpoints_mp_spacing { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_kpoints_mp_offset { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_gamma_directions { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_supercell_matrix { cells.push(v.to_cell()); }
        if let Some(v) = &self.supercell_kpoint_list { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_default() {
        let p = PhononParams::default();
        assert!(p.validate().is_ok());
    }

    #[test]
    fn test_validate_single_method_ok() {
        let p = PhononParams {
            phonon_kpoint_path: Some(PhononKpointPath { points: vec![] }),
            ..Default::default()
        };
        assert!(p.validate().is_ok());
    }

    #[test]
    fn test_validate_path_and_list_err() {
        let p = PhononParams {
            phonon_kpoint_path: Some(PhononKpointPath { points: vec![] }),
            phonon_kpoint_list: Some(PhononKpointList::builder().build()),
            ..Default::default()
        };
        assert!(p.validate().is_err());
    }

    #[test]
    fn test_validate_path_and_mp_grid_err() {
        let p = PhononParams {
            phonon_kpoint_path: Some(PhononKpointPath { points: vec![] }),
            phonon_kpoints_mp_grid: Some(PhononKpointsMpGrid([2, 2, 2])),
            ..Default::default()
        };
        assert!(p.validate().is_err());
    }

    #[test]
    fn test_validate_mp_grid_with_offset_ok() {
        let p = PhononParams {
            phonon_kpoints_mp_grid: Some(PhononKpointsMpGrid([2, 2, 2])),
            phonon_kpoints_mp_offset: Some(PhononKpointsMpOffset([0.0, 0.0, 0.0])),
            ..Default::default()
        };
        assert!(p.validate().is_ok());
    }

    #[test]
    fn test_validate_gamma_directions_with_method_ok() {
        let p = PhononParams {
            phonon_kpoint_path: Some(PhononKpointPath { points: vec![] }),
            phonon_gamma_directions: Some(PhononGammaDirections::builder().build()),
            ..Default::default()
        };
        assert!(p.validate().is_ok());
    }
}
