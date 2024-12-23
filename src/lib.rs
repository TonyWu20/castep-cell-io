#![allow(dead_code)]

pub mod cell_document;
mod formatting;
mod keywords;
mod parsing;

pub use cell_document::{
    params::{CastepParams, CastepParamsBuilder, CastepTask, EnergyCutoff, EnergyCutoffError},
    to_cell_document,
    units::length_units::*,
};
pub use keywords::{
    DocumentSections, KPointKeywords, KeywordType, LatticeBlockType, PositionsKeywords,
};
pub use parsing::{CellParseError, CellParser};

pub use parsing::helpers::{get_block_data, get_field_data, get_keyword};
