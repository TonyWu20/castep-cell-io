#![allow(dead_code)]
mod error;
pub mod format;
pub mod parse;
mod parser;
pub mod query;

pub use error::{CResult, Error};
pub use format::{to_string, to_string_many};
pub use parse::{FromBlock, FromCellFile, FromCellValue, FromKeyValue, parse};
pub use parser::parse_cell_file;
pub use parser::rich_error;
pub use query::{
    find_block, find_keyvalue, has_flag, row_as_f64_n, value_as_bool, value_as_f64, value_as_i32,
    value_as_str, value_as_string, value_as_u32,
};

// Intermediate representation for parsed data
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Cell<'a> {
    KeyValue(&'a str, CellValue<'a>),
    Block(&'a str, Vec<CellValue<'a>>),
    Flag(&'a str),
}

impl<'a> Cell<'a> {
    pub fn key(&self) -> &str {
        match self {
            Cell::KeyValue(key, _cell_value) => key,
            Cell::Block(name, _cell_value) => name,
            Cell::Flag(flag) => flag,
        }
    }
}

pub trait ToCellValue {
    fn to_cell_value(&self) -> CellValue;
}

pub trait ToCell {
    fn to_cell(&self) -> Cell;
}

pub trait ToCellFile {
    fn to_cell_file(&self) -> Vec<Cell>;
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum CellValue<'a> {
    Null,
    Bool(bool),
    Str(&'a str),
    String(String),
    UInt(u32),
    Int(i32),
    Float(f64),
    Array(Vec<CellValue<'a>>),
}

impl<'a> CellValue<'a> {
    pub fn as_array(&self) -> Option<&Vec<CellValue<'a>>> {
        if let Self::Array(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
