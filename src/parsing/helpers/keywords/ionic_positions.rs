use std::str::FromStr;

use castep_periodic_table::element::ElementSymbol;
use winnow::error::{ErrMode, InputError};
use winnow::stream::AsChar;
use winnow::{
    ascii::{alpha1, digit1, float, space0, space1, Caseless},
    combinator::{alt, preceded},
    error::{ContextError, StrContext},
    token::take_till,
    ModalResult, Parser,
};

use crate::cell_document::units::ParsableUnit;
use crate::cell_document::IonicPositionBlock;
use crate::parsing::helpers::block::get_block_data;
use crate::LengthUnit;
use crate::{
    cell_document::{IonicPosition, Mixture},
    keywords::{DocumentSections, PositionsKeywords},
    parsing::CellParseError,
};

fn assign_positions_frac<'s>(input: &mut &'s str) -> ModalResult<DocumentSections<'s>> {
    Caseless("positions_frac")
        .map(|_| DocumentSections::IonicPositions(PositionsKeywords::POSITIONS_FRAC))
        .parse_next(input)
}

fn assign_positions_abs<'s>(input: &mut &'s str) -> ModalResult<DocumentSections<'s>> {
    Caseless("positions_abs")
        .map(|_| DocumentSections::IonicPositions(PositionsKeywords::POSITIONS_ABS))
        .parse_next(input)
}

fn parse_mixture(input: &mut &str) -> ModalResult<Mixture> {
    preceded(space1, Caseless("mixture")).parse_next(input)?;
    take_till(0.., AsChar::is_dec_digit).parse_next(input)?;
    if let Ok((id, _, val)) =
        (digit1, space1, float::<&str, f64, InputError<&str>>).parse_next(input)
    {
        let id = id
            .parse::<usize>()
            .map_err(|_| ErrMode::Cut(ContextError::<StrContext>::new()))?;
        Ok(Mixture::new(id, val))
    } else {
        Err(ErrMode::Cut(ContextError::<StrContext>::new()))
    }
}

pub fn assign_positions_type<'s>(input: &mut &'s str) -> ModalResult<DocumentSections<'s>> {
    alt((assign_positions_frac, assign_positions_abs)).parse_next(input)
}

fn parse_line_of_position(input: &mut &str) -> ModalResult<IonicPosition> {
    let symbol = preceded(space0, alt((alpha1, digit1)))
        .map(|s| {
            ElementSymbol::from_str(s).map_err(|_| ErrMode::Cut(ContextError::<StrContext>::new()))
        })
        .parse_next(input)?;
    let mut coord: Vec<f64> = Vec::new();
    while let Ok(val) = preceded(space1, float::<&str, f64, InputError<&str>>).parse_next(input) {
        coord.push(val)
    }
    let coord: [f64; 3] = coord
        .try_into()
        .map_err(|_| ErrMode::Cut(ContextError::<StrContext>::new()))?;
    let mixture = parse_mixture(input).ok();
    let position = IonicPosition::new(symbol?, coord, mixture);
    Ok(position)
}

pub fn parse_ionic_positions(
    input: &mut &str,
    position_keyword: PositionsKeywords,
) -> Result<IonicPositionBlock, CellParseError> {
    let data = get_block_data(input).map_err(|_| CellParseError::GetBlockDataFailure)?;
    let mut lines: Vec<&str> = data
        .lines()
        // Filter out blank lines to prevent error in `parse_line_of_position`
        .filter(|s| !s.is_empty())
        .collect();
    let spin_polarised = data
        .lines()
        .any(|line| line.to_lowercase().contains("spin"));
    let unit = lines
        .iter_mut()
        .peekable()
        .peek()
        .and_then(|s| LengthUnit::parse_from_str(s).ok());
    let positions: Result<Vec<IonicPosition>, CellParseError> = if unit.is_some() {
        lines
            .iter_mut()
            .skip(1)
            .map(|line| parse_line_of_position(line).map_err(|_| CellParseError::Invalid))
            .collect()
    } else {
        lines
            .iter_mut()
            .map(|line| parse_line_of_position(line).map_err(|_| CellParseError::Invalid))
            .collect()
    };
    Ok(IonicPositionBlock::new(
        unit.unwrap_or_default(),
        positions?,
        position_keyword,
        spin_polarised,
    ))
}

#[cfg(test)]
mod test {

    use crate::{
        keywords::PositionsKeywords,
        parsing::helpers::keywords::ionic_positions::parse_ionic_positions,
    };

    #[test]
    fn keywords_position() {
        let mut input = "  C  0.0756034347004260  0.0756034355668187  0.5000000004346841
  C  0.1496332166229109  0.1496332194727908  0.5000000000710555
  C  0.2145289813410105  0.2145289823390266  0.4999999994942101 ##
  C  0.4243965644332829 -0.0000000008663758  0.5000000004346841
  C  0.3503667805273500 -0.0000000028498315  0.5000000000710555
  #V  0.3934837276229430  0.6065302751523840  0.5001896946183580 SPIN=  2.0000000000
%ENDBLOCK POSITIONS_FRAC

";
        let positions = parse_ionic_positions(&mut input, PositionsKeywords::POSITIONS_FRAC);
        println!("{:#?}", positions);
    }
}
