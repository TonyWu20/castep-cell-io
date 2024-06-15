use winnow::{
    ascii::{digit1, space1},
    combinator::preceded,
    PResult, Parser,
};

use crate::{
    data::{LCAOBasis, SpeciesLCAOStatesBlock},
    parsing::helpers::get_block_data,
    CellParseError,
};

use super::parse_element;

fn parse_line_of_lcao(input: &mut &str) -> PResult<LCAOBasis> {
    let element = parse_element(input)?;
    let lcao_states = preceded(space1, digit1)
        .map(|v: &str| v.parse::<u8>().expect("Fail to parse `u8`"))
        .parse_next(input)?;
    Ok(LCAOBasis::new(element, lcao_states))
}

pub fn parse_species_lcao_block(
    input: &mut &str,
) -> Result<SpeciesLCAOStatesBlock, CellParseError> {
    let data = get_block_data(input).map_err(|_| CellParseError::GetBlockDataFailure)?;
    let lcao_items = data
        .lines()
        .filter(|s| !s.is_empty())
        .map(|mut s| parse_line_of_lcao(&mut s))
        .collect::<PResult<Vec<LCAOBasis>>>()
        .map_err(|_| CellParseError::Invalid)?;
    Ok(SpeciesLCAOStatesBlock::new(lcao_items))
}
