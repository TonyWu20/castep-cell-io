//! Low-level parsing and formatting backend for CASTEP `.cell` / `.param` formats.
//!
//! This crate is the foundation for [`castep-cell-io`](https://crates.io/crates/castep-cell-io).
//! It provides the IR types, parser, formatter, and trait hierarchy. End users should use
//! `castep-cell-io` instead.
//!
//! # Parsing
//!
//! [`parse_cell_file`] tokenises text into `Vec<Cell<'_>>`. [`parse`] goes further and
//! deserialises directly into any type implementing [`FromCellFile`].
//!
//! # Formatting
//!
//! [`to_string_many`] serialises a `&[Cell<'_>]` back to CASTEP-formatted text.
//!
//! # Implementing a new keyword type
//!
//! ```
//! use castep_cell_fmt::{CellValue, CResult, Error};
//! use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
//! use castep_cell_fmt::query::value_as_str;
//!
//! #[derive(Debug, PartialEq)]
//! pub enum Task { SinglePoint }
//!
//! impl FromCellValue for Task {
//!     fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
//!         match value_as_str(value)?.to_ascii_lowercase().as_str() {
//!             "singlepoint" => Ok(Self::SinglePoint),
//!             other => Err(Error::Message(format!("unknown Task: {other}"))),
//!         }
//!     }
//! }
//!
//! impl FromKeyValue for Task {
//!     const KEY_NAME: &'static str = "TASK";
//!     fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
//!         Self::from_cell_value(value)
//!     }
//! }
//!
//! // Usage: parse from token slice
//! let tokens = castep_cell_fmt::parse_cell_file("TASK : SinglePoint").unwrap();
//! let task = Task::from_cells(&tokens).unwrap();
//! assert_eq!(task, Some(Task::SinglePoint));
//! ```
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
    find_block, find_block_any, find_keyvalue, has_flag, row_as_f64_n, value_as_bool, value_as_f64,
    value_as_i32, value_as_str, value_as_string, value_as_u32,
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
    fn to_cell_value(&self) -> CellValue<'_>;
}

pub trait ToCell {
    fn to_cell(&self) -> Cell<'_>;
}

pub trait ToCellFile {
    fn to_cell_file(&self) -> Vec<Cell<'_>>;
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
