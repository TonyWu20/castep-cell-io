mod ionic_positions;
mod lattice_parameters;
mod units;

#[allow(unused_imports)]
pub use ionic_positions::{IonicPosition, Mixture};
#[allow(unused_imports)]
pub use lattice_parameters::{LatticeABC, LatticeCart, LatticeParam, LatticeParamBlock};
#[allow(unused_imports)]
pub use units::*;

/// A structure to represent the `.cell` file.
#[derive(Debug)]
pub struct CellDocument {
    lattice: LatticeParamBlock,
    ionic_positions: Vec<IonicPosition>,
}

impl CellDocument {
    pub fn new(lattice: LatticeParamBlock, ionic_positions: Vec<IonicPosition>) -> Self {
        Self {
            lattice,
            ionic_positions,
        }
    }

    pub fn lattice(&self) -> LatticeParamBlock {
        self.lattice
    }

    pub fn ionic_positions(&self) -> &[IonicPosition] {
        self.ionic_positions.as_ref()
    }
}

pub trait CellData {}
