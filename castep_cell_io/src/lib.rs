//! CASTEP `.cell` and `.param` domain types.
//!
//! Parse, build, and format CASTEP input files using [`CellDocument`] and [`ParamDocument`].
//!
//! # Example
//!
//! ```no_run
//! use castep_cell_fmt::{parse, ToCellFile, format::to_string_many_spaced};
//! use castep_cell_io::CellDocument;
//!
//! let input = std::fs::read_to_string("structure.cell").unwrap();
//! let doc: CellDocument = parse(&input).unwrap();
//! let output = to_string_many_spaced(&doc.to_cell_file());
//! println!("{output}");
//! ```
#![allow(unused_imports, dead_code)]
pub mod cell;
pub mod param;
pub mod units;
mod cell_document;
mod param_document;

pub use cell_document::{CellDocument, CellDocumentBuilder, Lattice, Positions};
pub use param_document::{ParamDocument, ParamDocumentBuilder};
