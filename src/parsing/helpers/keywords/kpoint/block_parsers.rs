use winnow::{
    error::{AddContext, ContextError, ErrMode, StrContext, StrContextValue},
    stream::Stream,
    PResult,
};

use crate::{
    data::{BSKpointPath, KpointListBlock, KpointSettings, KpointTask, NCKpointSettings},
    parsing::helpers::get_block_data,
    CellParseError,
};

fn parse_line_of_kpoint(input: &mut &str) -> PResult<[f64; 4]> {
    let err_context = ContextError::<StrContext>::new().add_context(
        input,
        &input.checkpoint(),
        StrContext::Expected(StrContextValue::Description("Incorrect input: not `f64`")),
    );
    let kpoint: PResult<Vec<f64>> = input
        .split_whitespace()
        .map(|s| {
            s.parse::<f64>()
                .map_err(|_| ErrMode::Backtrack(err_context.clone()))
        })
        .collect();
    kpoint?
        .try_into()
        .map_err(|_| ErrMode::Backtrack(err_context))
}

fn parse_line_of_kpoint_path(input: &mut &str) -> PResult<[f64; 3]> {
    let err_context = ContextError::<StrContext>::new().add_context(
        input,
        &input.checkpoint(),
        StrContext::Expected(StrContextValue::Description("Incorrect input: not `f64`")),
    );
    let kpoint: PResult<Vec<f64>> = input
        .split_whitespace()
        .map(|s| {
            s.parse::<f64>()
                .map_err(|_| ErrMode::Backtrack(err_context.clone()))
        })
        .collect();
    kpoint?
        .try_into()
        .map_err(|_| ErrMode::Backtrack(err_context))
}

pub fn parse_kpoint_list(input: &mut &str) -> Result<KpointSettings, CellParseError> {
    let data = get_block_data(input).map_err(|_| CellParseError::GetBlockDataFailure)?;
    let kpoints = data
        .lines()
        .filter(|s| !s.is_empty())
        .map(|mut line| parse_line_of_kpoint(&mut line))
        .collect::<PResult<Vec<[f64; 4]>>>()
        .map_err(|_| CellParseError::Invalid)?;
    Ok(KpointSettings::List(KpointListBlock::new(
        KpointTask::SCF,
        kpoints,
    )))
}

pub fn parse_bs_kpoint_list(input: &mut &str) -> Result<NCKpointSettings, CellParseError> {
    let data = get_block_data(input).map_err(|_| CellParseError::GetBlockDataFailure)?;
    let bs_kpoints = data
        .lines()
        .filter(|s| !s.is_empty())
        .map(|mut line| parse_line_of_kpoint(&mut line))
        .collect::<PResult<Vec<[f64; 4]>>>()
        .map_err(|_| CellParseError::Invalid)?;
    Ok(NCKpointSettings::List(KpointListBlock::new(
        KpointTask::Spectral,
        bs_kpoints,
    )))
}

pub fn parse_bs_kpoint_path(input: &mut &str) -> Result<NCKpointSettings, CellParseError> {
    let data = get_block_data(input).map_err(|_| CellParseError::GetBlockDataFailure)?;
    let mut lines: Vec<&str> = data.lines().filter(|s| !s.is_empty()).collect();
    let kpoint_paths: Result<Vec<[f64; 3]>, CellParseError> = lines
        .iter_mut()
        .map(|line| parse_line_of_kpoint_path(line).map_err(|_| CellParseError::Invalid))
        .collect();
    Ok(NCKpointSettings::Path(BSKpointPath::new(kpoint_paths?)))
}
