use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromBlock, FromCellFile, ToCellFile, ToCell, FromKeyValue};

use super::bz_sampling_kpoints::*;

/// Optics and Magres k-point list parameters
///
/// Both fields are independent — they can coexist.
#[derive(Debug, Clone, Default, Builder)]
pub struct OpticsMagresParams {
    pub optics_kpoints_list: Option<OpticsKpointsList>,
    pub magres_kpoints_list: Option<MagresKpointsList>,
}

impl OpticsMagresParams {
    pub fn validate(self) -> Result<Self, String> {
        Ok(self)
    }
}

impl FromCellFile for OpticsMagresParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_optics_kpoints_list(OpticsKpointsList::from_cells(tokens).ok())
            .maybe_magres_kpoints_list(MagresKpointsList::from_cells(tokens).ok())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for OpticsMagresParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.optics_kpoints_list { cells.push(v.to_cell()); }
        if let Some(v) = &self.magres_kpoints_list { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_ok_any_combination() {
        let p = OpticsMagresParams::default();
        assert!(p.validate().is_ok());
        let p = OpticsMagresParams {
            optics_kpoints_list: Some(OpticsKpointsList { kpoints: vec![] }),
            ..Default::default()
        };
        assert!(p.validate().is_ok());
        let p = OpticsMagresParams {
            magres_kpoints_list: Some(MagresKpointsList { kpoints: vec![] }),
            ..Default::default()
        };
        assert!(p.validate().is_ok());
        let p = OpticsMagresParams {
            optics_kpoints_list: Some(OpticsKpointsList { kpoints: vec![] }),
            magres_kpoints_list: Some(MagresKpointsList { kpoints: vec![] }),
        };
        assert!(p.validate().is_ok());
    }
}
