mod ionic_positions;
mod kpoint_settings;
mod lattice_parameters;
pub mod units;

use std::{fs, io::Error, path::Path};

#[allow(unused_imports)]
pub use ionic_positions::{IonicPosition, IonicPositionBlock, Mixture};
#[allow(unused_imports)]
pub use lattice_parameters::{LatticeABC, LatticeCart, LatticeParam, LatticeParamBlock};
#[allow(unused_imports)]

/// A structure to represent the `.cell` file.
#[derive(Debug, Clone)]
pub struct CellDocument {
    lattice: LatticeParamBlock,
    ionic_positions: IonicPositionBlock,
}

impl CellDocument {
    pub fn new(lattice: LatticeParamBlock, ionic_positions: IonicPositionBlock) -> Self {
        Self {
            lattice,
            ionic_positions,
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
}

impl ToString for CellDocument {
    fn to_string(&self) -> String {
        [
            format!("{}", self.lattice),
            format!("{}", self.ionic_positions),
        ]
        .concat()
    }
}

pub trait CellData {}
