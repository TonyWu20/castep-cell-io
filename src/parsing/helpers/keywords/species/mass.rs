use winnow::{
    ascii::{float, space1},
    combinator::preceded,
    PResult, Parser,
};

use crate::{
    data::{SpeciesMass, SpeciesMassBlock},
    parsing::helpers::get_block_data,
    CellParseError,
};

use super::parse_element;

fn parse_line_of_mass(input: &mut &str) -> PResult<SpeciesMass> {
    let element = parse_element(input)?;
    let mass: f64 = preceded(space1, float).parse_next(input)?;
    Ok(SpeciesMass::new(element, mass))
}

pub fn parse_species_mass_block(input: &mut &str) -> Result<SpeciesMassBlock, CellParseError> {
    let data = get_block_data(input).map_err(|_| CellParseError::GetBlockDataFailure)?;
    let mass_items = data
        .lines()
        .filter(|s| !s.is_empty())
        .map(|mut s| parse_line_of_mass(&mut s))
        .collect::<PResult<Vec<SpeciesMass>>>()
        .map_err(|_| CellParseError::Invalid)?;
    Ok(SpeciesMassBlock::new(mass_items))
}
