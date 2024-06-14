mod lcao;
mod mass;
mod potentials;

use castep_periodic_table::element::ElementSymbol;

pub use lcao::{LCAOBasis, SpeciesLCAOStatesBlock};
pub use mass::{SpeciesMass, SpeciesMassBlock};
pub use potentials::{PseudoPot, SpeciesPotBlock};

pub trait SpeciesCharacter {
    type Output;
    fn from_element(element: ElementSymbol) -> Self::Output;
}
