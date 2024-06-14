use std::fmt::Display;

use castep_periodic_table::{data::ELEMENT_TABLE, element::ElementSymbol, element::LookupElement};

use crate::formatting::BlockDisplay;

use super::SpeciesCharacter;

#[derive(Debug, Clone)]
pub struct SpeciesLCAOStatesBlock {
    basis_sets: Vec<LCAOBasis>,
}

impl SpeciesLCAOStatesBlock {
    pub fn new(basis_sets: Vec<LCAOBasis>) -> Self {
        Self { basis_sets }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LCAOBasis {
    element: ElementSymbol,
    num_angular_momentum: u8,
}

impl LCAOBasis {
    pub fn new(element: ElementSymbol, num_angular_momentum: u8) -> Self {
        Self {
            element,
            num_angular_momentum,
        }
    }
}

impl SpeciesCharacter for LCAOBasis {
    type Output = LCAOBasis;

    fn from_element(element: ElementSymbol) -> Self::Output {
        let lcao = ELEMENT_TABLE.get_by_symbol(element).lcao();
        LCAOBasis::new(element, lcao)
    }
}

impl Display for LCAOBasis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:>8}{:9}",
            format!("{}", self.element),
            self.num_angular_momentum
        )
    }
}

impl BlockDisplay for SpeciesLCAOStatesBlock {
    fn block_tag(&self) -> String {
        "SPECIES_LCAO_STATES".to_string()
    }

    fn entries(&self) -> String {
        self.basis_sets
            .iter()
            .map(|b| format!("{b}"))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Display for SpeciesLCAOStatesBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content())
    }
}
