use std::fmt::Display;

use castep_periodic_table::{data::ELEMENT_TABLE, element::ElementSymbol, element::LookupElement};

use crate::formatting::BlockDisplay;

use super::SpeciesCharacter;

#[derive(Debug, Clone)]
pub struct PseudoPot {
    element: ElementSymbol,
    pot_file: String,
}

#[derive(Debug, Clone)]
pub struct SpeciesPotBlock {
    items: Vec<PseudoPot>,
}

impl SpeciesPotBlock {
    pub fn new(items: Vec<PseudoPot>) -> Self {
        Self { items }
    }
}

impl PseudoPot {
    pub fn new(element: ElementSymbol, pot_file: String) -> Self {
        Self { element, pot_file }
    }
}

impl SpeciesCharacter for PseudoPot {
    type Output = PseudoPot;

    fn from_element(element: ElementSymbol) -> Self::Output {
        Self::new(
            element,
            ELEMENT_TABLE.get_by_symbol(element).potential().to_string(),
        )
    }
}

impl Display for PseudoPot {
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
