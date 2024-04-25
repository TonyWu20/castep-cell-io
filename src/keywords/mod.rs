mod constraints;
mod kpoint;
mod lattice;
mod positions;
mod species;
mod symmetry;

pub use lattice::*;

#[derive(Debug)]
pub struct CellDocument {
    lattice: LatticeParam,
}

#[derive(Debug, Default)]
pub enum ItemTypes {
    #[default]
    Block,
    Physical,
    Logical,
    D,
    RealVector,
    IntegerVector,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum BlockKeywords {
    LATTICE_CART,
    LATTICE_ABC,
    POSITIONS_FRAC,
    POSITIONS_ABC,
    KPOINT_LIST,
    SPECTRAL_KPOINT_PATH,
    SPECTRAL_KPOINT_LIST,
    PHONON_KPOINT_PATH,
    PHONON_KPOINT_LIST,
    PHONON_GAMMA_DIRECTIONS,
    SYMMETRY_OPS,
    IONIC_CONSTRAINTS,
    CELL_CONSTRAINTS,
    SPECIES_MASS,
    SPECIES_POT,
    SPECIES_LCAO_STATES,
    EXTERNAL_PRESSURE,
    IONIC_VELOCITIES,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum CellLatticeVectors {
    LATTICE_ABC,
    LATTICE_CART,
}
