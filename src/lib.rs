#![allow(dead_code)]

pub use parser::parse_cell_file;
pub use parser::rich_error;

// Intermediate representation for parsed data
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum CellValue<'a> {
    Null,
    Bool(bool),
    Str(&'a str),
    UInt(u32),
    Int(i32),
    Float(f64),
    Array(Vec<CellValue<'a>>),
}

mod de;
mod error;
mod parser;
