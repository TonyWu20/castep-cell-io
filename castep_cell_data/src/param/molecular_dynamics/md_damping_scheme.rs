use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;

/// Controls the damping scheme used during damped molecular dynamics.
///
/// Keyword type: String
///
/// Default: MdDampingScheme::Independent
///
/// Example:
/// MD_DAMPING_SCHEME : Coupled
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MdDampingScheme {
    /// Calculates optimal damping parameters according to the independent modes algorithm
    Independent,
    /// Calculates optimal damping parameters according to the coupled modes algorithm
    Coupled,
    /// Performs steepest descent dynamics
    SteepestDescents,
}

impl Default for MdDampingScheme {
    fn default() -> Self {
        Self::Independent // Default is Independent
    }
}

impl FromCellValue for MdDampingScheme {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "independent" => Ok(Self::Independent),
            "coupled" => Ok(Self::Coupled),
            "steepestdescents" => Ok(Self::SteepestDescents),
            other => Err(Error::Message(format!("unknown MdDampingScheme: {other}"))),
        }
    }
}

impl FromKeyValue for MdDampingScheme {
    const KEY_NAME: &'static str = "MD_DAMPING_SCHEME";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MdDampingScheme {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_DAMPING_SCHEME", self.to_cell_value())
    }
}

impl ToCellValue for MdDampingScheme {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                MdDampingScheme::Independent => "Independent",
                MdDampingScheme::Coupled => "Coupled",
                MdDampingScheme::SteepestDescents => "SteepestDescents",
            }
            .to_string(),
        )
    }
}


