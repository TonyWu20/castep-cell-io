mod ionic_positions;
mod lattice_parameters;

#[allow(unused_imports)]
pub use ionic_positions::{IonicPosition, Mixture};
#[allow(unused_imports)]
pub use lattice_parameters::{LatticeABC, LatticeCart, LatticeParam};

/// A structure to represent the `.cell` file.
#[derive(Debug)]
pub struct CellDocument {
    lattice: LatticeParam,
    ionic_positions: Vec<IonicPosition>,
}

impl CellDocument {
    pub fn new(lattice: LatticeParam, ionic_positions: Vec<IonicPosition>) -> Self {
        Self {
            lattice,
            ionic_positions,
        }
    }

    pub fn lattice(&self) -> LatticeParam {
        self.lattice
    }

    pub fn ionic_positions(&self) -> &[IonicPosition] {
        self.ionic_positions.as_ref()
    }
}

pub trait CellData {}
