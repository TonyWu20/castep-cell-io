use winnow::{ascii::Caseless, combinator::alt, PResult, Parser};

use crate::keywords::{DocumentSections, KPointKeywords};

pub fn assign_kpoint_list_block<'s>(input: &mut &'s str) -> PResult<DocumentSections<'s>> {
    alt((Caseless("kpoint_list"), Caseless("kpoints_list")))
        .map(|_| DocumentSections::KPoint(KPointKeywords::KPOINT_LIST))
        .parse_next(input)
}

pub fn assign_bs_kpoint_list_block<'s>(input: &mut &'s str) -> PResult<DocumentSections<'s>> {
    alt((
        Caseless("bs_kpoint_list"),
        Caseless("bs_kpoints_list"),
        Caseless("spectral_kpoint_list"),
        Caseless("spectral_kpoints_list"),
    ))
    .map(|_| DocumentSections::KPoint(KPointKeywords::SPECTRAL_KPOINT_LIST))
    .parse_next(input)
}

pub fn assign_kpoint_mp_grid_field<'s>(input: &mut &'s str) -> PResult<DocumentSections<'s>> {
    alt((Caseless("kpoint_mp_grid"), Caseless("kpoints_mp_grid")))
        .map(|_| DocumentSections::KPoint(KPointKeywords::KPOINT_MP_GRID))
        .parse_next(input)
}

pub fn assign_kpoint_mp_spacing_field<'s>(input: &mut &'s str) -> PResult<DocumentSections<'s>> {
    alt((
        Caseless("kpoint_mp_spacing"),
        Caseless("kpoints_mp_spacing"),
    ))
    .map(|_| DocumentSections::KPoint(KPointKeywords::KPOINT_MP_SPACING))
    .parse_next(input)
}

pub fn assign_kpoint_mp_offset_field<'s>(input: &mut &'s str) -> PResult<DocumentSections<'s>> {
    alt((Caseless("kpoint_mp_offset"), Caseless("kpoints_mp_offset")))
        .map(|_| DocumentSections::KPoint(KPointKeywords::KPOINT_MP_OFFSET))
        .parse_next(input)
}

pub fn assign_kpoint_bs_path_spacing_field<'s>(
    input: &mut &'s str,
) -> PResult<DocumentSections<'s>> {
    alt((
        Caseless("bs_kpoint_path_spacing"),
        Caseless("spectral_kpoint_path_spacing"),
        Caseless("bs_kpoints_path_spacing"),
        Caseless("spectral_kpoints_path_spacing"),
    ))
    .map(|_| DocumentSections::KPoint(KPointKeywords::SPECTRAL_KPOINT_PATH_SPACING))
    .parse_next(input)
}

pub fn assign_bs_kpoint_path_block<'s>(input: &mut &'s str) -> PResult<DocumentSections<'s>> {
    alt((Caseless("bs_kpoint_path"), Caseless("spectral_kpoint_path")))
        .map(|_| DocumentSections::KPoint(KPointKeywords::SPECTRAL_KPOINT_PATH))
        .parse_next(input)
}
