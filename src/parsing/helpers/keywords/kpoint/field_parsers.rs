use crate::{
    data::{units::ParsableUnit, KpointMPGrid, KpointMPSpacing, KpointSettings, KpointTask},
    parsing::helpers::get_field_data,
    CellParseError, InvLengthUnit,
};

pub fn parse_kpoint_mp_grid_field(input: &mut &str) -> Result<KpointSettings, CellParseError> {
    let data = get_field_data(input).map_err(|_| CellParseError::GetFieldDataFailure)?;
    let grid: [u32; 3] = data
        .split_whitespace()
        .map(|n| {
            n.parse::<u32>()
                .map_err(|_| CellParseError::UnexpectedValueType)
        })
        .collect::<Result<Vec<u32>, CellParseError>>()?
        .try_into()
        .map_err(|_| CellParseError::UnexpectedLength)?;
    Ok(KpointSettings::MPGrid(KpointMPGrid::new(
        KpointTask::SCF,
        grid,
    )))
}

pub fn parse_kpoint_mp_spacing_field(input: &mut &str) -> Result<KpointSettings, CellParseError> {
    let data = get_field_data(input).map_err(|_| CellParseError::GetFieldDataFailure)?;
    let spacing = data
        .split_whitespace()
        .next()
        .and_then(|val| {
            val.parse::<f64>()
                .map_err(|_| CellParseError::UnexpectedValueType)
                .ok()
        })
        .ok_or(CellParseError::Invalid)?;
    let unit = match data.split_whitespace().nth(1) {
        Some(unit_str) => InvLengthUnit::parse_from_str(unit_str)?,
        None => InvLengthUnit::default(),
    };
    Ok(KpointSettings::MPSpacing(KpointMPSpacing::new(
        KpointTask::SCF,
        spacing,
        unit,
    )))
}

pub fn parse_bs_kpoint_path_spacing_field(
    _input: &mut &str,
) -> Result<KpointSettings, CellParseError> {
    todo!()
}

pub fn parse_kpoint_offset_field(_input: &mut &str) -> Result<KpointSettings, CellParseError> {
    todo!()
}
