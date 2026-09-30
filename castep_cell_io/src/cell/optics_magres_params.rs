use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromBlock, FromCellFile, ToCellFile, ToCell, FromKeyValue};

use super::bz_sampling_kpoints::*;
use super::jcoupling_site::JcouplingSite;

/// Optics and Magres k-point list parameters
///
/// Both fields are independent — they can coexist.
#[derive(Debug, Clone, Default, Builder)]
pub struct OpticsMagresParams {
    pub optics_kpoints_list: Option<OpticsKpointsList>,
    pub magres_kpoints_list: Option<MagresKpointsList>,
    /// 6.11: perturbing site for a MAGRES J-coupling calculation.
    pub jcoupling_site: Option<JcouplingSite>,
}

impl OpticsMagresParams {
    pub fn validate(self) -> Result<Self, String> {
        // CASTEP checks the J-coupling ion number against the species' ion
        // count (`ni < 1` aborts in `Fundamental/cell.f90`); the lower bound
        // is self-contained and validated here.
        if let Some(site) = &self.jcoupling_site
            && site.ion_number == 0
        {
            return Err("JCOUPLING_SITE ion number must be >= 1".into());
        }
        Ok(self)
    }
}

impl FromCellFile for OpticsMagresParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_optics_kpoints_list(OpticsKpointsList::from_cells(tokens).ok())
            .maybe_magres_kpoints_list(MagresKpointsList::from_cells(tokens).ok())
            .maybe_jcoupling_site(JcouplingSite::from_cells(tokens).ok().flatten())
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
        if let Some(v) = &self.jcoupling_site { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::species::Species;

    #[test]
    fn test_jcoupling_site_from_inline_parse() {
        // The 6.11 string keyword is written inline, not as a block.
        let input = "JCOUPLING_SITE : Fe 2\n";
        let tokens = castep_cell_fmt::parse_cell_file(input).unwrap();
        let params = OpticsMagresParams::from_cell_file(&tokens).unwrap();
        let site = params.jcoupling_site.expect("jcoupling_site should be present");
        assert_eq!(site.species, Species::Symbol("Fe".to_string()));
        assert_eq!(site.ion_number, 2);
    }

    #[test]
    fn test_jcoupling_site_zero_ion_rejected_by_validate() {
        let params = OpticsMagresParams::builder()
            .jcoupling_site(
                JcouplingSite::builder()
                    .species(Species::Symbol("Fe".to_string()))
                    .ion_number(0)
                    .build(),
            )
            .build();
        assert!(params.validate().is_err());
    }

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
            ..Default::default()
        };
        assert!(p.validate().is_ok());
    }
}
