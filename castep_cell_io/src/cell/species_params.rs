use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromBlock, FromCellFile, ToCellFile, ToCell, FromKeyValue};

use super::species::*;

/// Species properties
///
/// Contains all species-related blocks: masses, pseudopotentials, LCAO states,
/// charges, Hubbard U corrections, and SEDC custom parameters.
/// All fields are independent — they can coexist.
#[derive(Debug, Clone, Default, Builder)]
pub struct SpeciesParams {
    pub species_mass: Option<SpeciesMass>,
    pub species_pot: Option<SpeciesPot>,
    pub species_lcao_states: Option<SpeciesLcaoStates>,
    pub species_q: Option<SpeciesQ>,
    pub hubbard_u: Option<HubbardU>,
    pub sedc_custom_params: Option<SedcCustomParams>,
}

impl SpeciesParams {
    pub fn validate(self) -> Result<Self, String> {
        Ok(self)
    }
}

impl FromCellFile for SpeciesParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_species_mass(SpeciesMass::from_cells(tokens).ok())
            .maybe_species_pot(SpeciesPot::from_cells(tokens).ok())
            .maybe_species_lcao_states(SpeciesLcaoStates::from_cells(tokens).ok())
            .maybe_species_q(SpeciesQ::from_cells(tokens).ok())
            .maybe_hubbard_u(HubbardU::from_cells(tokens).ok())
            .maybe_sedc_custom_params(SedcCustomParams::from_cells(tokens).ok())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for SpeciesParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.species_mass { cells.push(v.to_cell()); }
        if let Some(v) = &self.species_pot { cells.push(v.to_cell()); }
        if let Some(v) = &self.species_lcao_states { cells.push(v.to_cell()); }
        if let Some(v) = &self.species_q { cells.push(v.to_cell()); }
        if let Some(v) = &self.hubbard_u { cells.push(v.to_cell()); }
        if let Some(v) = &self.sedc_custom_params { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_default() {
        let p = SpeciesParams::default();
        assert!(p.validate().is_ok());
    }
}
