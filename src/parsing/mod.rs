mod error;
mod helpers;

pub use error::CellParseError;

use crate::{
    data::{CellDocument, IonicPosition, LatticeParamBlock},
    keywords::{DocumentSections, KeywordType},
    parsing::helpers::{
        current_sections, get_block_data, get_field_data, parse_ionic_positions,
        parse_lattice_param,
    },
};

#[derive(Debug)]
pub struct CellParser<'a> {
    input: &'a str,
    lattice_param: Option<LatticeParamBlock>,
    ionic_positions: Option<Vec<IonicPosition>>,
}

impl<'a> From<&'a str> for CellParser<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            input: value,
            lattice_param: None,
            ionic_positions: None,
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
                    let positions = parse_ionic_positions(&mut self.input)?;
                    self.ionic_positions = Some(positions);
                }
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
            if self.lattice_param.is_some() && self.ionic_positions.is_some() {
                println!("Lattice parameters and atomic coordinates have been collected.");
                println!("Parsing Finished");
                break;
            }
        }
        let cell_doc = CellDocument::new(
            self.lattice_param.unwrap(),
            self.ionic_positions.as_ref().unwrap().to_owned(),
        );

        Ok(cell_doc)
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
    }
}
