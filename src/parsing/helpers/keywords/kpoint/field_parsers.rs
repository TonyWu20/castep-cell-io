use crate::{data::KpointSettings, parsing::helpers::get_field_data, CellParseError};

pub fn parse_kpoint_mp_grid_field(input: &mut &str) -> Result<KpointSettings, CellParseError> {
    let data = get_field_data(input).map_err(|_| CellParseError::GetFieldDataFailure)?;
    todo!()
}

pub fn parse_kpoint_mp_spacing_field(input: &mut &str) -> Result<KpointSettings, CellParseError> {
    todo!()
}

pub fn parse_bs_kpoint_path_spacing_field(
    input: &mut &str,
) -> Result<KpointSettings, CellParseError> {
    todo!()
}

pub fn parse_kpoint_offset_field(input: &mut &str) -> Result<KpointSettings, CellParseError> {
    todo!()
}
