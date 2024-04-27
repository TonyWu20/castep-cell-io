mod error;
mod helpers;

pub use error::CellParseError;

use crate::{
    data::{CellDocument, IonicPosition, LatticeParam},
    keywords::DocumentSections,
    parsing::helpers::{current_sections, parse_ionic_positions, parse_lattice_param},
};

#[derive(Debug)]
pub struct CellParser<'a> {
    input: &'a str,
    lattice_param: Option<LatticeParam>,
    ionic_positions: Option<Vec<IonicPosition>>,
}

impl<'a> CellParser<'a> {
    pub fn from_str(input: &'a str) -> Self {
        Self {
            input,
            lattice_param: None,
            ionic_positions: None,
        }
    }

    pub fn parse(&mut self) -> Result<CellDocument, CellParseError> {
        while let Ok(section) = current_sections(&mut self.input) {
            match section {
                DocumentSections::CellLatticeVectors(lat_type) => {
                    let param = parse_lattice_param(&mut self.input, lat_type)?;
                    self.lattice_param = Some(param);
                }
                DocumentSections::IonicPositions(_) => {
                    let positions = parse_ionic_positions(&mut self.input)?;
                    self.ionic_positions = Some(positions);
                }
                _ => {
                    if self.lattice_param.is_some() && self.ionic_positions.is_some() {
                        break;
                    }
                }
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
        let mut cell_parser = CellParser::from_str(&input);
        let cell_doc = cell_parser.parse().unwrap();
        println!("{:#?}", cell_doc);
    }
}
