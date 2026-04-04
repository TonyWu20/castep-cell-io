use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;

/// Specifies the verbosity of reporting of population analysis results.
///
/// Keyword type: String
///
/// Default: PopnWrite::Enhanced
///
/// Example:
/// POPN_WRITE : SUMMARY
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PopnWrite {
    /// No output
    None,
    /// Summary only
    Minimal,
    /// Same as MINIMAL
    Summary,
    /// Summary and orbital-resolved PDOS components
    #[default]
    Enhanced,
    /// As ENHANCED and S and T matrices
    Verbose,
}

impl FromCellValue for PopnWrite {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "none" => Ok(Self::None),
            "minimal" => Ok(Self::Minimal),
            "summary" => Ok(Self::Summary),
            "enhanced" => Ok(Self::Enhanced),
            "verbose" => Ok(Self::Verbose),
            other => Err(Error::Message(format!("unknown PopnWrite: {other}"))),
        }
    }
}

impl FromKeyValue for PopnWrite {
    const KEY_NAME: &'static str = "POPN_WRITE";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for PopnWrite {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("POPN_WRITE", self.to_cell_value())
    }
}

impl ToCellValue for PopnWrite {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                PopnWrite::None => "NONE",
                PopnWrite::Minimal => "MINIMAL",
                PopnWrite::Summary => "SUMMARY",
                PopnWrite::Enhanced => "ENHANCED",
                PopnWrite::Verbose => "VERBOSE",
            }
            .to_string(),
        )
    }
}

