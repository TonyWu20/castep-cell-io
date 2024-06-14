use winnow::{
    combinator::alt,
    error::{AddContext, ContextError, ErrMode, StrContext, StrContextValue},
    stream::Stream,
    PResult, Parser,
};

use crate::{
    data::{BSKpointPath, KpointListBlock, KpointSettings, KpointTask, NCKpointSettings},
    keywords::DocumentSections,
    parsing::helpers::get_block_data,
    CellParseError,
};

pub(crate) mod assignments;

use assignments::{
    assign_bs_kpoint_list_block, assign_kpoint_list_block, assign_kpoint_mp_grid_field,
    assign_kpoint_mp_offset_field, assign_kpoint_mp_spacing_field,
};

pub fn assign_kpoint_list_type<'s>(input: &mut &'s str) -> PResult<DocumentSections<'s>> {
    alt((assign_kpoint_list_block, assign_bs_kpoint_list_block)).parse_next(input)
}

pub fn assign_kpoint_field_settings<'s>(input: &mut &'s str) -> PResult<DocumentSections<'s>> {
    alt((
        assign_kpoint_mp_grid_field,
        assign_kpoint_mp_spacing_field,
        assign_kpoint_mp_offset_field,
    ))
    .parse_next(input)
}

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
    let mut lines: Vec<&str> = data.lines().filter(|s| !s.is_empty()).collect();
    let kpoints: Result<Vec<[f64; 4]>, CellParseError> = lines
        .iter_mut()
        .map(|line| parse_line_of_kpoint(line).map_err(|_| CellParseError::Invalid))
        .collect();
    Ok(KpointSettings::List(KpointListBlock::new(
        KpointTask::SCF,
        kpoints?,
    )))
}

pub fn parse_kpoint_path(input: &mut &str) -> Result<NCKpointSettings, CellParseError> {
    let data = get_block_data(input).map_err(|_| CellParseError::GetBlockDataFailure)?;
    let mut lines: Vec<&str> = data.lines().filter(|s| !s.is_empty()).collect();
    let kpoint_paths: Result<Vec<[f64; 3]>, CellParseError> = lines
        .iter_mut()
        .map(|line| parse_line_of_kpoint_path(line).map_err(|_| CellParseError::Invalid))
        .collect();
    Ok(NCKpointSettings::Path(BSKpointPath::new(kpoint_paths?)))
}

// TODO: other parsers for fields
