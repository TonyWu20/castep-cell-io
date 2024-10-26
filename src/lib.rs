#![allow(dead_code)]

mod calc_setup;
mod cell_document;
mod formatting;
mod keywords;
mod parsing;

pub use calc_setup::SeedfileGenerator;
pub use cell_document::{
    params::{CastepParams, CastepParamsBuilder, CastepTask},
    units::length_units::*,
    CellDocument, IonicPosition, IonicPositionBlock, LatticeABC, LatticeCart, LatticeParam,
    LatticeParamBlock,
};
pub use keywords::{
    DocumentSections, KPointKeywords, KeywordType, LatticeBlockType, PositionsKeywords,
};
pub use parsing::{CellParseError, CellParser};

pub use parsing::helpers::{get_block_data, get_field_data, get_keyword};
