mod error;
mod helpers;

pub use error::CellParseError;

use crate::{
    data::{CellDocument, CellEntries, IonicPositionBlock, LatticeParamBlock},
    keywords::{DocumentSections, KeywordType, SpeciesKeywords},
    parsing::helpers::{
        current_sections, get_block_data, get_field_data, parse_ionic_positions, parse_kpoint_list,
        parse_lattice_param,
    },
};

use self::helpers::parse_species_mass_block;

#[derive(Debug)]
pub struct CellParser<'a> {
    input: &'a str,
    lattice_param: Option<LatticeParamBlock>,
    ionic_positions: Option<IonicPositionBlock>,
    other_entries: Vec<CellEntries>,
}

impl<'a> From<&'a str> for CellParser<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            input: value,
            lattice_param: None,
            ionic_positions: None,
            other_entries: Vec::new(),
        }
    }
}

impl<'a> CellParser<'a> {
    pub fn parse(&mut self) -> Result<CellDocument, CellParseError> {
        while let Ok(section) = current_sections(&mut self.input) {
            match section {
                DocumentSections::CellLatticeVectors(lat_type) => {
                    println!("{:?}", lat_type);
                    let param = parse_lattice_param(&mut self.input, lat_type)?;
                    self.lattice_param = Some(param);
                }
                DocumentSections::IonicPositions(pos_type) => {
                    println!("{:?}", pos_type);
                    let positions = parse_ionic_positions(&mut self.input, pos_type)?;
                    self.ionic_positions = Some(positions);
                }
                DocumentSections::KPoint(kpt_type) => {
                    println!("{:?}", kpt_type);
                    let kpt_list = parse_kpoint_list(&mut self.input)?;
                    let kpt_settings = CellEntries::KpointSettings(kpt_list);
                    self.other_entries.push(kpt_settings);
                }
                DocumentSections::Species(spec_keyword) => match spec_keyword {
                    SpeciesKeywords::SPECIES_LCAO_STATES => {
                        println!("{:?}", spec_keyword)
                    }
                    SpeciesKeywords::SPECIES_MASS => {
                        let spec_mass = parse_species_mass_block(&mut self.input)?;
                        self.other_entries.push(CellEntries::SpeciesMass(spec_mass));
                    }
                    SpeciesKeywords::SPECIES_POT => {
                        println!("{:?}", spec_keyword)
                    }
                },
                DocumentSections::Misc(ref keyword) => {
                    match keyword {
                        KeywordType::Block(_) => {
                            get_block_data(&mut self.input)
                                .map_err(|_| CellParseError::GetBlockDataFailure)?;
                        }
                        KeywordType::Field(_) => {
                            get_field_data(&mut self.input)
                                .map_err(|_| CellParseError::GetBlockDataFailure)?;
                        }
                    }
                    println!("{:?}", section)
                }
                _ => {
                    println!("{:?}", section)
                }
            }
        }
        if self.lattice_param.is_some() && self.ionic_positions.is_some() {
            let mut cell_doc = CellDocument::new(
                self.lattice_param.unwrap(),
                self.ionic_positions.as_ref().unwrap().to_owned(),
            );
            cell_doc.set_entries(Some(self.other_entries.clone()));
            Ok(cell_doc)
        } else {
            Err(CellParseError::RequiredSectionMissing)
        }
    }
}

#[cfg(test)]
mod test {
    use std::{fs, path::Path};

    use super::CellParser;

    #[test]
    fn test_cell_parser() {
        let root = env!("CARGO_MANIFEST_DIR");
        let path = Path::new(root).join("SAC_GDY_V.cell");
        let input = fs::read_to_string(path).unwrap();
        let mut cell_parser = CellParser::from(input.as_str());
        let cell_doc = cell_parser.parse();
        println!("Parse status: {:?}", cell_doc.is_ok());
        let path = Path::new(root).join("SAC_GDY_V_test.cell");
        let input = fs::read_to_string(path).unwrap();
        let mut cell_parser = CellParser::from(input.as_str());
        let cell_doc = cell_parser.parse();
        println!("Parse status: {:?}", cell_doc.is_ok());
        println!("{}", cell_doc.unwrap());
    }
}
