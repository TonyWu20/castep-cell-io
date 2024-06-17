#[allow(unused_imports)]
mod params;
mod sections;
pub mod units;

use std::{fmt::Display, fs, io::Error, path::Path};

use castep_periodic_table::element::ElementSymbol;
pub use sections::external_fields::{ExtEFieldBlock, ExtPressureBlock};
pub use sections::ionic_positions::{IonicPosition, IonicPositionBlock, Mixture};
pub use sections::kpoint_settings::*;
pub use sections::lattice_parameters::{LatticeABC, LatticeCart, LatticeParam, LatticeParamBlock};
pub use sections::species_characters::{
    LCAOBasis, SpeciesLCAOStatesBlock, SpeciesMass, SpeciesMassBlock, SpeciesPot, SpeciesPotBlock,
};

pub use params::{CastepParams, CastepParamsBuilder, CastepTask};
pub use sections::constraints::{FixAllCell, FixAllIons, FixCom, IonicConstraintsBlock};

/// A structure to represent the `.cell` file.
#[derive(Debug, Clone)]
pub struct CellDocument {
    lattice: LatticeParamBlock,
    ionic_positions: IonicPositionBlock,
    entries: Option<Vec<CellEntries>>,
}

impl CellDocument {
    pub fn new(lattice: LatticeParamBlock, ionic_positions: IonicPositionBlock) -> Self {
        Self {
            lattice,
            ionic_positions,
            entries: None,
        }
    }

    pub fn lattice(&self) -> LatticeParamBlock {
        self.lattice
    }

    pub fn ionic_positions(&self) -> &IonicPositionBlock {
        &self.ionic_positions
    }
    pub fn write_out<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        fs::write(path, self.to_string())
    }

    pub fn ionic_positions_mut(&mut self) -> &mut IonicPositionBlock {
        &mut self.ionic_positions
    }

    pub fn entries(&self) -> Option<&Vec<CellEntries>> {
        self.entries.as_ref()
    }

    pub fn entries_mut(&mut self) -> &mut Option<Vec<CellEntries>> {
        &mut self.entries
    }

    pub fn set_entries(&mut self, entries: Option<Vec<CellEntries>>) {
        self.entries = entries;
    }
    pub fn get_elements(&self) -> Vec<ElementSymbol> {
        let mut symbols: Vec<ElementSymbol> = self
            .ionic_positions()
            .positions()
            .iter()
            .map(|pos| pos.symbol())
            .collect();
        symbols.sort();
        symbols.dedup();
        symbols
    }
}

impl Display for CellDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let entries = self
            .entries()
            .map(|e| {
                e.iter()
                    .map(|item| format!("{}", item))
                    .collect::<Vec<String>>()
                    .join("\n")
            })
            .unwrap_or_default();
        let content = [
            format!("{}", self.lattice),
            format!("{}", self.ionic_positions),
            entries,
        ]
        .concat();
        write!(f, "{}", content)
    }
}

#[derive(Debug, Clone)]
pub enum CellEntries {
    KpointSettings(KpointSettings),
    NCKpointSettings(NCKpointSettings),
    /// This keyword controls whether or not all of the lattice parameters remain fixed during relaxation or molecular dynamics.
    FixAllCell(FixAllCell),
    FixAllIons(FixAllIons),
    /// This keyword controls whether or not the center of mass of the ions remains fixed during relaxation or molecular dynamics.
    FixCom(FixCom),
    IonicConstraints(IonicConstraintsBlock),
    ExtEfield(ExtEFieldBlock),
    ExtPressure(ExtPressureBlock),
    SpeciesMass(SpeciesMassBlock),
    SpeciesPot(SpeciesPotBlock),
    SpeciesLCAOStates(SpeciesLCAOStatesBlock),
}

impl Display for CellEntries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            CellEntries::KpointSettings(v) => format!("{v}"),
            CellEntries::NCKpointSettings(v) => format!("{v}"),
            CellEntries::FixAllCell(v) => format!("{v}"),
            CellEntries::FixAllIons(v) => format!("{v}"),
            CellEntries::FixCom(v) => format!("{v}"),
            CellEntries::IonicConstraints(v) => format!("{v}"),
            CellEntries::ExtEfield(v) => format!("{v}"),
            CellEntries::ExtPressure(v) => format!("{v}"),
            CellEntries::SpeciesMass(v) => format!("{v}"),
            CellEntries::SpeciesPot(v) => format!("{v}"),
            CellEntries::SpeciesLCAOStates(v) => format!("{v}"),
        };
        write!(f, "{content}")
    }
}