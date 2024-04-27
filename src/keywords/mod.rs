mod constraints;
mod kpoint;
mod lattice;
mod positions;
mod species;
mod symmetry;

pub use lattice::*;
pub use positions::*;

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
#[non_exhaustive]
#[derive(Debug)]
pub enum DocumentSections {
    CellLatticeVectors(LatticeBlockType),
    IonicPositions(PositionsKeywords),
    KPoint,
    Symmetry,
    Species,
    Misc,
}
