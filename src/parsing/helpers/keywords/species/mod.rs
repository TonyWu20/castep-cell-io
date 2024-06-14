use std::str::FromStr;

use castep_periodic_table::element::ElementSymbol;
use winnow::{
    ascii::{alpha1, digit1, float, space0, space1, Caseless},
    combinator::{alt, preceded},
    PResult, Parser,
};

use crate::{
    data::{SpeciesMass, SpeciesMassBlock},
    keywords::{DocumentSections, SpeciesKeywords},
    parsing::helpers::get_block_data,
    CellParseError,
};

pub fn assign_species_type<'a, 's>(input: &mut &'s str) -> PResult<DocumentSections<'s>> {
    let assignment = |tag_name: &'a str, keyword: SpeciesKeywords| {
        Caseless(tag_name).map(move |_| DocumentSections::Species(keyword))
    };
    alt((
        assignment("species_mass", SpeciesKeywords::SPECIES_MASS),
        assignment("species_pot", SpeciesKeywords::SPECIES_POT),
        assignment("species_lcao_states", SpeciesKeywords::SPECIES_LCAO_STATES),
    ))
    .parse_next(input)
}

fn parse_line_of_mass(input: &mut &str) -> PResult<SpeciesMass> {
    let element = alt((preceded(space0, alpha1), preceded(space0, digit1)))
        .map(|s| {
            ElementSymbol::from_str(s).expect("Error in input element symbol or atomic number")
        })
        .parse_next(input)?;
    let mass: f64 = preceded(space1, float).parse_next(input)?;
    Ok(SpeciesMass::new(element, mass))
}

pub fn parse_species_mass_block(input: &mut &str) -> Result<SpeciesMassBlock, CellParseError> {
    let data = get_block_data(input).map_err(|_| CellParseError::GetBlockDataFailure)?;
    let mut lines: Vec<&str> = data.lines().filter(|s| !s.is_empty()).collect();
    let mass_items = lines
        .iter_mut()
        .map(parse_line_of_mass)
        .collect::<PResult<Vec<SpeciesMass>>>()
        .map_err(|_| CellParseError::Invalid)?;
    Ok(SpeciesMassBlock::new(mass_items))
}
