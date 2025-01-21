use std::fmt::Display;

use castep_periodic_table::{data::ELEMENT_TABLE, element::ElementSymbol, element::LookupElement};

use crate::formatting::BlockDisplay;

use super::{SpeciesBlock, SpeciesCharacter, SpeciesEntry};

#[derive(Debug, Clone)]
pub struct SpeciesPot {
    element: ElementSymbol,
    pot_file: String,
}

impl SpeciesEntry for SpeciesPot {
    type Item = String;

    fn element(&self) -> &ElementSymbol {
        &self.element
    }

    fn item(&self) -> &Self::Item {
        &self.pot_file
    }
}

#[derive(Debug, Clone)]
pub struct SpeciesPotBlock {
    items: Vec<SpeciesPot>,
}

impl SpeciesBlock for SpeciesPotBlock {
    type ItemOutput = SpeciesPot;

    fn items(&self) -> &[Self::ItemOutput] {
        &self.items
    }

    fn items_mut(&mut self) -> &mut Vec<Self::ItemOutput> {
        &mut self.items
    }
}

impl SpeciesPotBlock {
    pub fn new(items: Vec<SpeciesPot>) -> Self {
        Self { items }
    }
    pub fn from_elements(elements: &[ElementSymbol]) -> Self {
        Self::new(
            elements
                .iter()
                .map(|&elm| SpeciesPot::from_element(elm))
                .collect(),
        )
    }
}

impl SpeciesPot {
    pub fn new(element: ElementSymbol, pot_file: String) -> Self {
        Self { element, pot_file }
    }
}

impl SpeciesCharacter for SpeciesPot {
    type Output = SpeciesPot;

    fn from_element(element: ElementSymbol) -> Self::Output {
        Self::new(
            element,
            ELEMENT_TABLE.get_by_symbol(element).potential().to_string(),
        )
    }
}

impl Display for SpeciesPot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>8}  {}", format!("{}", self.element), self.pot_file)
    }
}

impl BlockDisplay for SpeciesPotBlock {
    fn block_tag(&self) -> String {
        "SPECIES_POT".to_string()
    }

    fn entries(&self) -> String {
        self.items
            .iter()
            .map(|p| format!("{p}"))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Display for SpeciesPotBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content())
    }
}
