mod constraints;
mod kpoint;
mod lattice;
mod positions;
mod species;
mod symmetry;

pub use kpoint::KPointKeywords;
pub use lattice::LatticeBlockType;
pub use positions::PositionsKeywords;
pub use species::SpeciesKeywords;

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

#[derive(Debug)]
pub enum KeywordType<'s> {
    Block(&'s str),
    Field(&'s str),
}

#[allow(non_camel_case_types)]
#[non_exhaustive]
#[derive(Debug)]
pub enum DocumentSections<'s> {
    CellLatticeVectors(LatticeBlockType),
    IonicPositions(PositionsKeywords),
    KPoint(KPointKeywords),
    Symmetry,
    Species(SpeciesKeywords),
    Misc(KeywordType<'s>),
}
