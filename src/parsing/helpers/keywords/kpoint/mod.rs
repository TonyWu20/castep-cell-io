use winnow::{combinator::alt, ModalResult, Parser};

use crate::keywords::DocumentSections;

mod assignments;
mod block_parsers;
// TODO: other parsers for fields
mod field_parsers;

use assignments::{
    assign_bs_kpoint_list_block, assign_bs_kpoint_path_block, assign_kpoint_bs_path_spacing_field,
    assign_kpoint_list_block, assign_kpoint_mp_grid_field, assign_kpoint_mp_offset_field,
    assign_kpoint_mp_spacing_field,
};

pub use block_parsers::{parse_bs_kpoint_list, parse_bs_kpoint_path, parse_kpoint_list};
pub use field_parsers::{parse_kpoint_mp_grid_field, parse_kpoint_mp_spacing_field};

pub fn assign_kpoint_block_type<'s>(input: &mut &'s str) -> ModalResult<DocumentSections<'s>> {
    alt((
        assign_kpoint_list_block,
        assign_bs_kpoint_list_block,
        assign_bs_kpoint_path_block,
    ))
    .parse_next(input)
}

pub fn assign_kpoint_field_settings<'s>(input: &mut &'s str) -> ModalResult<DocumentSections<'s>> {
    alt((
        assign_kpoint_mp_grid_field,
        assign_kpoint_mp_spacing_field,
        assign_kpoint_mp_offset_field,
        assign_kpoint_bs_path_spacing_field,
    ))
    .parse_next(input)
}
