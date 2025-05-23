use crate::{
    cell_document::{CellDocument, CellEntries, CellEssentials},
    keywords::{
        DocumentSections, KPointKeywords, KeywordType, LatticeBlockType, PositionsKeywords,
        SpeciesKeywords,
    },
    parsing::helpers::{
        current_sections, get_block_data, get_field_data, parse_bs_kpoint_list,
        parse_ionic_positions, parse_kpoint_list, parse_lattice_param, parse_species_mass_block,
        parse_species_pot_block,
    },
    CellParseError, CellParser,
};

use super::helpers::{
    parse_bs_kpoint_path, parse_kpoint_mp_grid_field, parse_kpoint_mp_spacing_field,
    parse_species_lcao_block,
};

impl<'a, S: AsRef<str>> From<&'a S> for CellParser<'a> {
    fn from(value: &'a S) -> Self {
        Self {
            input: value.as_ref(),
            lattice_param: None,
            ionic_positions: None,
            other_entries: Vec::new(),
        }
    }
}

impl CellParser<'_> {
    pub fn parse(&mut self) -> Result<CellDocument, CellParseError> {
        while let Ok(section) = current_sections(&mut self.input) {
            match section {
                DocumentSections::CellLatticeVectors(lat_keyword) => {
                    self.parse_lattice_param_section(lat_keyword)?;
                }
                DocumentSections::IonicPositions(pos_keyword) => {
                    self.parse_ionic_pos_section(pos_keyword)?;
                }
                DocumentSections::KPoint(kpt_keyword) => {
                    let kpt_setting = self.parse_kpt_section(kpt_keyword)?;
                    self.other_entries.push(kpt_setting);
                }
                DocumentSections::Species(spec_keyword) => {
                    let entry = self.parse_species_section(spec_keyword)?;
                    self.other_entries.push(entry);
                }
                DocumentSections::Misc(ref misc_keyword) => match misc_keyword {
                    KeywordType::Block(_) => {
                        get_block_data(&mut self.input)
                            .map_err(|_| CellParseError::GetBlockDataFailure)?;
                        #[cfg(debug_assertions)]
                        dbg!(&self.input);
                    }
                    KeywordType::Field(field_kw) => {
                        let field_data = get_field_data(&mut self.input)
                            .map_err(|_| CellParseError::GetBlockDataFailure)?;
                        #[cfg(debug_assertions)]
                        {
                            dbg!((field_kw, field_data));
                        }
                    }
                },
                DocumentSections::End => {
                    break;
                }
                _ => {
                    #[cfg(debug_assertions)]
                    println!("{:?}", section)
                }
            }
        }
        if self.lattice_param.is_some() && self.ionic_positions.is_some() {
            let model_description = CellEssentials::new(
                self.lattice_param.unwrap(),
                self.ionic_positions.as_ref().unwrap().to_owned(),
            );
            let mut cell_doc = CellDocument::new(model_description);
            cell_doc.set_entries(Some(self.other_entries.clone()));
            // Default to true whether `SPIN` is found in parsing or not.
            cell_doc
                .model_description_mut()
                .ionic_pos_block_mut()
                .set_spin_polarised(true);
            Ok(cell_doc)
        } else {
            Err(CellParseError::RequiredSectionMissing)
        }
    }
    fn parse_lattice_param_section(
        &mut self,
        lat_keyword: LatticeBlockType,
    ) -> Result<(), CellParseError> {
        let param = parse_lattice_param(&mut self.input, lat_keyword)?;
        self.lattice_param = Some(param);
        Ok(())
    }
    fn parse_ionic_pos_section(
        &mut self,
        pos_keyword: PositionsKeywords,
    ) -> Result<(), CellParseError> {
        let positions = parse_ionic_positions(&mut self.input, pos_keyword)?;
        self.ionic_positions = Some(positions);
        Ok(())
    }
    fn parse_species_section(
        &mut self,
        species_keyword: SpeciesKeywords,
    ) -> Result<CellEntries, CellParseError> {
        match species_keyword {
            SpeciesKeywords::SPECIES_LCAO_STATES => Ok(CellEntries::SpeciesLCAOStates(
                parse_species_lcao_block(&mut self.input)?,
            )),
            SpeciesKeywords::SPECIES_MASS => Ok(CellEntries::SpeciesMass(
                parse_species_mass_block(&mut self.input)?,
            )),
            SpeciesKeywords::SPECIES_POT => Ok(CellEntries::SpeciesPot(parse_species_pot_block(
                &mut self.input,
            )?)),
        }
    }
    fn parse_kpt_section(
        &mut self,
        kpt_keyword: KPointKeywords,
    ) -> Result<CellEntries, CellParseError> {
        match kpt_keyword {
            KPointKeywords::KPOINT_LIST => Ok(CellEntries::KpointSettings(parse_kpoint_list(
                &mut self.input,
            )?)),
            KPointKeywords::KPOINT_MP_GRID => Ok(CellEntries::KpointSettings(
                parse_kpoint_mp_grid_field(&mut self.input)?,
            )),
            KPointKeywords::KPOINT_MP_SPACING => Ok(CellEntries::KpointSettings(
                parse_kpoint_mp_spacing_field(&mut self.input)?,
            )),
            KPointKeywords::KPOINT_MP_OFFSET => todo!(),
            KPointKeywords::SPECTRAL_KPOINT_LIST => Ok(CellEntries::NCKpointSettings(
                parse_bs_kpoint_list(&mut self.input)?,
            )),
            KPointKeywords::SPECTRAL_KPOINT_PATH => Ok(CellEntries::NCKpointSettings(
                parse_bs_kpoint_path(&mut self.input)?,
            )),
            KPointKeywords::SPECTRAL_KPOINT_PATH_SPACING => todo!(),
        }
    }
}
