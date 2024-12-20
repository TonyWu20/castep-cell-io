#[allow(unused_imports)]
mod chemrust_impl;
pub mod params;
mod sections;
pub mod units;

use std::{fmt::Display, fs, io::Error, path::Path};

use castep_periodic_table::element::ElementSymbol;
use chemrust_core::data::lattice::CrystalModel;
use chemrust_core::data::symmetry::SymmetryInfo;

pub use sections::constraints::{FixAllCell, FixCom, IonicConstraintsBlock};
pub use sections::external_fields::{ExtEFieldBlock, ExtPressureBlock};
pub use sections::ionic_positions::{IonicPosition, IonicPositionBlock, Mixture};
pub use sections::kpoint_settings::*;
pub use sections::lattice_parameters::{LatticeABC, LatticeCart, LatticeParam, LatticeParamBlock};
pub use sections::species_characters::{
    LCAOBasis, SpeciesLCAOStatesBlock, SpeciesMass, SpeciesMassBlock, SpeciesPot, SpeciesPotBlock,
};

pub use chemrust_impl::to_cell_document;
pub use sections::CellEntries;
pub use sections::CellEssentials;

/// A structure to represent the `.cell` file.
#[derive(Debug, Clone)]
pub struct CellDocument {
    model_description: CellEssentials,
    other_entries: Option<Vec<CellEntries>>,
}

impl CrystalModel for CellDocument {
    type LatticeData = LatticeParamBlock;

    type AtomData = IonicPositionBlock;

    fn get_cell_parameters(&self) -> &Self::LatticeData {
        self.model_description().lattice_block()
    }

    fn get_atom_data(&self) -> &Self::AtomData {
        self.model_description().ionic_pos_block()
    }

    fn get_cell_parameters_mut(&mut self) -> &mut Self::LatticeData {
        self.model_description_mut().lattice_block_mut()
    }

    fn get_atom_data_mut(&mut self) -> &mut Self::AtomData {
        self.model_description_mut().ionic_pos_block_mut()
    }
}

impl SymmetryInfo for CellDocument {
    fn get_space_group_it_num(&self) -> u8 {
        if matches!(
            self.other_entries().and_then(|entries| {
                entries
                    .iter()
                    .find(|entry| matches!(entry, CellEntries::SymmetryOps(_)))
            }),
            Some(CellEntries::SymmetryOps(..))
        ) {
            todo!()
        } else {
            1_u8
        }
    }

    fn make_symmetry(&self) -> bool {
        matches!(
            self.other_entries().and_then(|entries| {
                entries
                    .iter()
                    .find(|entry| matches!(entry, CellEntries::SymmetryOps(_)))
            }),
            Some(CellEntries::SymmetryOps(..))
        )
    }
}

impl CellDocument {
    pub fn new(model_description: CellEssentials) -> Self {
        Self {
            model_description,
            other_entries: None,
        }
    }

    pub fn write_out<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        fs::write(path, self.to_string())
    }

    pub fn other_entries(&self) -> Option<&Vec<CellEntries>> {
        self.other_entries.as_ref()
    }

    pub fn other_entries_mut(&mut self) -> &mut Option<Vec<CellEntries>> {
        &mut self.other_entries
    }

    pub fn set_entries(&mut self, entries: Option<Vec<CellEntries>>) {
        self.other_entries = entries;
    }
    pub fn get_elements(&self) -> Vec<ElementSymbol> {
        let mut symbols: Vec<ElementSymbol> = self
            .model_description()
            .ionic_pos_block()
            .positions()
            .iter()
            .map(|pos| pos.symbol())
            .collect();
        symbols.sort();
        symbols.dedup();
        symbols
    }

    pub fn model_description(&self) -> &CellEssentials {
        &self.model_description
    }

    pub fn model_description_mut(&mut self) -> &mut CellEssentials {
        &mut self.model_description
    }

    pub fn set_model_description(&mut self, model_description: CellEssentials) {
        self.model_description = model_description;
    }
}

impl Display for CellDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let entries = self
            .other_entries()
            .map(|e| {
                e.iter()
                    .map(|item| format!("{}", item))
                    .collect::<Vec<String>>()
                    .join("\n")
            })
            .unwrap_or_default();
        let content = [
            format!("{}", self.model_description().lattice_block()),
            format!("{}", self.model_description().ionic_pos_block()),
            entries,
        ]
        .concat();
        write!(f, "{}", content)
    }
}
