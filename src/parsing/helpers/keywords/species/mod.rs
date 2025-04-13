use std::str::FromStr;

use castep_periodic_table::element::ElementSymbol;
use winnow::{
    ascii::{alpha1, digit1, space0, Caseless},
    combinator::{alt, preceded},
    ModalResult, Parser,
};

use crate::keywords::{DocumentSections, SpeciesKeywords};

mod lcao_states;
mod mass;
mod potentials;

pub use lcao_states::parse_species_lcao_block;
pub use mass::parse_species_mass_block;
pub use potentials::parse_species_pot_block;

pub fn assign_species_type<'s>(input: &mut &'s str) -> ModalResult<DocumentSections<'s>> {
    let assignment = |tag_name: &'s str, keyword: SpeciesKeywords| {
        Caseless(tag_name).map(move |_| DocumentSections::Species(keyword))
    };
    alt((
        assignment("species_mass", SpeciesKeywords::SPECIES_MASS),
        assignment("species_pot", SpeciesKeywords::SPECIES_POT),
        assignment("species_lcao_states", SpeciesKeywords::SPECIES_LCAO_STATES),
    ))
    .parse_next(input)
}

fn parse_element(input: &mut &str) -> ModalResult<ElementSymbol> {
    alt((preceded(space0, alpha1), preceded(space0, digit1)))
        .map(|s| {
            ElementSymbol::from_str(s).expect("Error in input element symbol or atomic number")
        })
        .parse_next(input)
}
