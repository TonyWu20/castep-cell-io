mod ionic_positions;
mod lattice_parameters;
mod units;

#[allow(unused_imports)]
pub use ionic_positions::{IonicPosition, IonicPositionBlock, Mixture};
#[allow(unused_imports)]
pub use lattice_parameters::{LatticeABC, LatticeCart, LatticeParam, LatticeParamBlock};
#[allow(unused_imports)]
pub use units::*;

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
    pub fn write_out(&self) -> String {
        [
            format!("{}", self.lattice),
            format!("{}", self.ionic_positions),
        ]
        .concat()
    }
}

pub trait CellData {}
