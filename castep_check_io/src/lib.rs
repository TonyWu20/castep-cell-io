//! Typed reader and writer for CASTEP binary model files.
//!
//! Reads, edits, and writes `.check` (full model, with wavefunction) and
//! `.castep_bin` (analysis model, no wavefunction) files. See ADR-0005.

pub mod error;
pub mod record;
pub mod params;
pub mod cell;
pub mod electronic;
pub mod document;
pub mod version;

pub use error::{CheckError, CResult, FileVersion};
pub use record::{char_field, char_str, parse_c128s, parse_f64s, parse_i32s, parse_logical4s, Complex, RecordReader, RecordWriter};
pub use document::CheckDocument;
pub use params::{LayoutConfig, ParamsLayout};
pub use version::{check_file_version, gate, gate_open, parse_version_record, target_file_version, target_version_record, CheckKind, MIN_SUPPORTED};
