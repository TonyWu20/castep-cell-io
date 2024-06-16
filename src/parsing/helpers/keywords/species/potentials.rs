use winnow::{
    ascii::{space1, till_line_ending},
    combinator::preceded,
    PResult, Parser,
};

use crate::{
    cell_document::{SpeciesPot, SpeciesPotBlock},
    parsing::helpers::get_block_data,
    CellParseError,
};

use super::parse_element;

fn parse_line_of_pot(input: &mut &str) -> PResult<SpeciesPot> {
    let element = parse_element(input)?;
    let pot_file = preceded(space1, till_line_ending).parse_next(input)?;
    Ok(SpeciesPot::new(element, pot_file.to_string()))
}

pub fn parse_species_pot_block(input: &mut &str) -> Result<SpeciesPotBlock, CellParseError> {
    let data = get_block_data(input).map_err(|_| CellParseError::GetBlockDataFailure)?;
    let pot_items = data
        .lines()
        .filter(|s| !s.is_empty())
        .map(|mut s| parse_line_of_pot(&mut s))
        .collect::<PResult<Vec<SpeciesPot>>>()
        .map_err(|_| CellParseError::Invalid)?;
    Ok(SpeciesPotBlock::new(pot_items))
}
