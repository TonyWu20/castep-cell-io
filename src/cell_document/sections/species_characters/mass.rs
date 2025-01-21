use std::fmt::Display;

use castep_periodic_table::{
    data::ELEMENT_TABLE,
    element::{ElementSymbol, LookupElement},
};

use crate::formatting::BlockDisplay;

use super::{SpeciesBlock, SpeciesCharacter, SpeciesEntry};

#[derive(Debug, Clone)]
pub struct SpeciesMassBlock {
    items: Vec<SpeciesMass>,
}

impl SpeciesBlock for SpeciesMassBlock {
    type ItemOutput = SpeciesMass;

    fn items(&self) -> &[Self::ItemOutput] {
        &self.items
    }

    fn items_mut(&mut self) -> &mut Vec<Self::ItemOutput> {
        &mut self.items
    }
}

impl SpeciesMassBlock {
    pub fn new(items: Vec<SpeciesMass>) -> Self {
        Self { items }
    }
    pub fn from_elements(elements: &[ElementSymbol]) -> Self {
        Self::new(
            elements
                .iter()
                .map(|&elm| SpeciesMass::from_element(elm))
                .collect(),
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SpeciesMass {
    element: ElementSymbol,
    mass: f64,
}

impl SpeciesEntry for SpeciesMass {
    type Item = f64;

    fn element(&self) -> &ElementSymbol {
        &self.element
    }

    fn item(&self) -> &Self::Item {
        &self.mass
    }
}

impl SpeciesMass {
    pub fn new(element: ElementSymbol, mass: f64) -> Self {
        Self { element, mass }
    }
}

impl SpeciesCharacter for SpeciesMass {
    type Output = SpeciesMass;

    fn from_element(element: ElementSymbol) -> Self::Output {
        SpeciesMass::new(element, ELEMENT_TABLE.get_by_symbol(element).mass())
    }
}

impl Display for SpeciesMass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>8}{:17.10}", format!("{}", self.element), self.mass)
    }
}

impl BlockDisplay for SpeciesMassBlock {
    fn block_tag(&self) -> String {
        "SPECIES_MASS".to_string()
    }

    fn entries(&self) -> String {
        self.items
            .iter()
            .map(|m| format!("{m}"))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Display for SpeciesMassBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content())
    }
}
