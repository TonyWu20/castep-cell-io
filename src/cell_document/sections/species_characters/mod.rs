mod lcao;
mod mass;
mod potentials;

use castep_periodic_table::element::ElementSymbol;

pub use lcao::{LCAOBasis, SpeciesLCAOStatesBlock};
pub use mass::{SpeciesMass, SpeciesMassBlock};
pub use potentials::{SpeciesPot, SpeciesPotBlock};

pub trait SpeciesCharacter {
    type Output;
    fn from_element(element: ElementSymbol) -> Self::Output;
}

pub trait SpeciesBlock {
    type ItemOutput: SpeciesEntry;
    fn items(&self) -> &[Self::ItemOutput];
    fn items_mut(&mut self) -> &mut Vec<Self::ItemOutput>;
}

pub trait SpeciesEntry {
    type Item;
    fn element(&self) -> &ElementSymbol;
    fn item(&self) -> &Self::Item;
}
