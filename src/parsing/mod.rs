mod cell_parser;
mod error;
pub mod helpers;

pub use error::CellParseError;

use crate::cell_document::{CellEntries, IonicPositionBlock, LatticeParamBlock};

#[derive(Debug)]
pub struct CellParser<'a> {
    input: &'a str,
    lattice_param: Option<LatticeParamBlock>,
    ionic_positions: Option<IonicPositionBlock>,
    other_entries: Vec<CellEntries>,
}

#[cfg(test)]
mod test {
    use std::{fs, path::Path};

    use super::CellParser;

    #[test]
    fn test_cell_parser() {
        let root = env!("CARGO_MANIFEST_DIR");
        let path = Path::new(root).join("FePcCOOH_N1.cell");
        let input = fs::read_to_string(path).unwrap();
        let mut cell_parser = CellParser::from(&input);
        let cell_doc = cell_parser.parse();
        println!("Parse status: {:?}", cell_doc.is_ok());
        let path = Path::new(root).join("SAC_GDY_V_test.cell");
        let input = fs::read_to_string(path).unwrap();
        let mut cell_parser = CellParser::from(&input);
        let cell_doc = cell_parser.parse();
        println!("Parse status: {:?}", cell_doc.is_ok());
        println!("{}", cell_doc.unwrap());
    }
}
