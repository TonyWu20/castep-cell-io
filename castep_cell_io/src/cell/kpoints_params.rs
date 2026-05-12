use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromBlock, FromCellFile, ToCellFile, ToCell, FromKeyValue};

use super::bz_sampling_kpoints::*;

/// SCF k-point sampling parameters
///
/// This group contains the k-point sampling settings for the SCF calculation.
/// Only one of [`KpointsList`], [`KpointsMpGrid`], or [`KpointsMpSpacing`] may be present.
/// [`KpointsMpOffset`] is an optional companion to MP grid/spacing methods.
#[derive(Debug, Clone, Default, Builder)]
pub struct KpointsParams {
    pub kpoints_list: Option<KpointsList>,
    pub kpoints_mp_grid: Option<KpointsMpGrid>,
    pub kpoints_mp_spacing: Option<KpointsMpSpacing>,
    pub kpoints_mp_offset: Option<KpointsMpOffset>,
}

impl KpointsParams {
    /// Validates k-point mutual exclusion: at most one of {list, mp_grid, mp_spacing}.
    pub fn validate(self) -> Result<Self, String> {
        let count = [self.kpoints_list.is_some(), self.kpoints_mp_grid.is_some(), self.kpoints_mp_spacing.is_some()]
            .into_iter()
            .filter(|&b| b)
            .count();
        if count > 1 {
            return Err("Only one of KPOINTS_LIST, KPOINTS_MP_GRID, or KPOINTS_MP_SPACING may be present".into());
        }
        Ok(self)
    }
}

impl FromCellFile for KpointsParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_kpoints_list(KpointsList::from_cells(tokens).ok())
            .maybe_kpoints_mp_grid(KpointsMpGrid::from_cells(tokens).ok().flatten())
            .maybe_kpoints_mp_spacing(KpointsMpSpacing::from_cells(tokens).ok().flatten())
            .maybe_kpoints_mp_offset(KpointsMpOffset::from_cells(tokens).ok().flatten())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for KpointsParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.kpoints_list { cells.push(v.to_cell()); }
        if let Some(v) = &self.kpoints_mp_grid { cells.push(v.to_cell()); }
        if let Some(v) = &self.kpoints_mp_spacing { cells.push(v.to_cell()); }
        if let Some(v) = &self.kpoints_mp_offset { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kpoints_params_default() {
        let params = KpointsParams::default();
        assert!(params.kpoints_list.is_none());
        assert!(params.kpoints_mp_grid.is_none());
        assert!(params.kpoints_mp_spacing.is_none());
        assert!(params.kpoints_mp_offset.is_none());
    }

    #[test]
    fn test_kpoints_params_builder() {
        let params = KpointsParams::builder().build();
        assert!(params.kpoints_list.is_none());
    }

    #[test]
    fn test_kpoints_params_to_cell_file_empty() {
        let params = KpointsParams::default();
        let cells = params.to_cell_file();
        assert_eq!(cells.len(), 0);
    }

    #[test]
    fn test_validate_single_method_ok() {
        let params = KpointsParams {
            kpoints_list: Some(KpointsList { kpts: vec![] }),
            ..Default::default()
        };
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_validate_rejects_multiple_methods() {
        let params = KpointsParams {
            kpoints_list: Some(KpointsList { kpts: vec![] }),
            kpoints_mp_grid: Some(KpointsMpGrid([1, 1, 1])),
            ..Default::default()
        };
        assert!(params.validate().is_err());
    }

    #[test]
    fn test_validate_mp_grid_with_offset_ok() {
        let params = KpointsParams {
            kpoints_mp_grid: Some(KpointsMpGrid([1, 1, 1])),
            kpoints_mp_offset: Some(KpointsMpOffset([0.0, 0.0, 0.0])),
            ..Default::default()
        };
        assert!(params.validate().is_ok());
    }
}
