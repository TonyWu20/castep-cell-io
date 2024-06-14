use winnow::{ascii::Caseless, combinator::alt, PResult, Parser};

use crate::keywords::{DocumentSections, SpeciesKeywords};

mod mass;
mod potentials;

pub use mass::parse_species_mass_block;

pub fn assign_species_type<'s>(input: &mut &'s str) -> PResult<DocumentSections<'s>> {
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
