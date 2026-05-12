use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromBlock, FromCellFile, ToCellFile, ToCell, FromKeyValue};

use super::bz_sampling_kpoints::*;

/// Spectral/BS k-point parameters
///
/// Contains both BS_ and SPECTRAL_ prefixed k-point types. At most one spectral
/// k-point method (path, list, mp_grid, or mp_spacing) may be present, regardless
/// of BS_ or SPECTRAL_ prefix.
///
/// The BS_ types are kept for backward compatibility (legacy CASTEP input files).
/// Serialization emits the native name for each type (BS_ stays BS_, SPECTRAL_ stays SPECTRAL_).
#[derive(Debug, Clone, Default, Builder)]
pub struct SpectralParams {
    pub bs_kpoint_path: Option<BsKpointPath>,
    pub bs_kpoints_list: Option<BSKpointList>,
    pub bs_kpoint_path_spacing: Option<BsKpointPathSpacing>,
    pub spectral_kpoint_path: Option<SpectralKpointPath>,
    pub spectral_kpoints_list: Option<SpectralKpointsList>,
    pub spectral_kpoint_path_spacing: Option<SpectralKpointPathSpacing>,
    pub spectral_kpoints_mp_grid: Option<SpectralKpointsMpGrid>,
    pub spectral_kpoints_mp_spacing: Option<SpectralKpointsMpSpacing>,
    pub spectral_kpoints_mp_offset: Option<SpectralKpointsMpOffset>,
}

impl SpectralParams {
    /// Validates spectral k-point mutual exclusion.
    ///
    /// At most one spectral method may be present (regardless of BS_ or SPECTRAL_ prefix).
    /// `spectral_kpoints_mp_offset` has no mutex constraint (companion to mp_grid/mp_spacing).
    pub fn validate(self) -> Result<Self, String> {
        let method_count = [
            self.bs_kpoint_path.is_some(),
            self.bs_kpoints_list.is_some(),
            self.bs_kpoint_path_spacing.is_some(),
            self.spectral_kpoint_path.is_some(),
            self.spectral_kpoints_list.is_some(),
            self.spectral_kpoint_path_spacing.is_some(),
            self.spectral_kpoints_mp_grid.is_some(),
            self.spectral_kpoints_mp_spacing.is_some(),
        ]
        .into_iter()
        .filter(|&b| b)
        .count();

        if method_count > 1 {
            return Err("Only one spectral k-point method may be present (SPECTRAL_KPOINT_PATH, SPECTRAL_KPOINTS_LIST, SPECTRAL_KPOINTS_MP_GRID, or SPECTRAL_KPOINTS_MP_SPACING; BS_ variants count as the same method)".into());
        }
        Ok(self)
    }
}

impl FromCellFile for SpectralParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_bs_kpoint_path(BsKpointPath::from_cells(tokens).ok())
            .maybe_bs_kpoints_list(BSKpointList::from_cells(tokens).ok())
            .maybe_bs_kpoint_path_spacing(BsKpointPathSpacing::from_cells(tokens).ok().flatten())
            .maybe_spectral_kpoint_path(SpectralKpointPath::from_cells(tokens).ok())
            .maybe_spectral_kpoints_list(SpectralKpointsList::from_cells(tokens).ok())
            .maybe_spectral_kpoint_path_spacing(SpectralKpointPathSpacing::from_cells(tokens).ok().flatten())
            .maybe_spectral_kpoints_mp_grid(SpectralKpointsMpGrid::from_cells(tokens).ok().flatten())
            .maybe_spectral_kpoints_mp_spacing(SpectralKpointsMpSpacing::from_cells(tokens).ok().flatten())
            .maybe_spectral_kpoints_mp_offset(SpectralKpointsMpOffset::from_cells(tokens).ok().flatten())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for SpectralParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.bs_kpoint_path { cells.push(v.to_cell()); }
        if let Some(v) = &self.bs_kpoints_list { cells.push(v.to_cell()); }
        if let Some(v) = &self.bs_kpoint_path_spacing { cells.push(v.to_cell()); }
        if let Some(v) = &self.spectral_kpoint_path { cells.push(v.to_cell()); }
        if let Some(v) = &self.spectral_kpoints_list { cells.push(v.to_cell()); }
        if let Some(v) = &self.spectral_kpoint_path_spacing { cells.push(v.to_cell()); }
        if let Some(v) = &self.spectral_kpoints_mp_grid { cells.push(v.to_cell()); }
        if let Some(v) = &self.spectral_kpoints_mp_spacing { cells.push(v.to_cell()); }
        if let Some(v) = &self.spectral_kpoints_mp_offset { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_default() {
        let p = SpectralParams::default();
        assert!(p.validate().is_ok());
    }

    #[test]
    fn test_validate_single_method_ok() {
        let p = SpectralParams {
            spectral_kpoint_path: Some(SpectralKpointPath::builder().build()),
            ..Default::default()
        };
        assert!(p.validate().is_ok());
    }

    #[test]
    fn test_validate_bs_and_spectral_path_err() {
        let p = SpectralParams {
            bs_kpoint_path: Some(BsKpointPath::builder().build()),
            spectral_kpoint_path: Some(SpectralKpointPath::builder().build()),
            ..Default::default()
        };
        assert!(p.validate().is_err());
    }

    #[test]
    fn test_validate_bs_path_and_spectral_list_err() {
        let p = SpectralParams {
            bs_kpoint_path: Some(BsKpointPath::builder().build()),
            spectral_kpoints_list: Some(SpectralKpointsList::builder().build()),
            ..Default::default()
        };
        assert!(p.validate().is_err());
    }

    #[test]
    fn test_validate_mp_grid_with_offset_ok() {
        let p = SpectralParams {
            spectral_kpoints_mp_grid: Some(SpectralKpointsMpGrid([1, 1, 1])),
            spectral_kpoints_mp_offset: Some(SpectralKpointsMpOffset([0.0, 0.0, 0.0])),
            ..Default::default()
        };
        assert!(p.validate().is_ok());
    }

    #[test]
    fn test_validate_mp_grid_and_mp_spacing_err() {
        let p = SpectralParams {
            spectral_kpoints_mp_grid: Some(SpectralKpointsMpGrid([1, 1, 1])),
            spectral_kpoints_mp_spacing: Some(SpectralKpointsMpSpacing {
                value: 0.1,
                unit: None,
            }),
            ..Default::default()
        };
        assert!(p.validate().is_err());
    }
}
