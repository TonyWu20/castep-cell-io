use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromKeyValue, CResult};
use castep_cell_io::query::value_as_str;

/// Controls the method used to minimize electronic states.
///
/// Keyword type: String
///
/// Default: ElectronicMinimizer::Cg
///
/// Example:
/// ELECTRONIC_MINIMIZER : SD
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ElectronicMinimizer {
    /// Minimizer takes up to 10 SD steps
    Sd,
    /// Minimizer takes one SD step followed by up to 4 CG steps
    Cg,
    #[doc(hidden)]
    RmmDiis,
}

impl Default for ElectronicMinimizer {
    fn default() -> Self {
        Self::Cg
    }
}

impl FromKeyValue for ElectronicMinimizer {
    const KEY_NAME: &'static str = "ELECTRONIC_MINIMIZER";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "sd" => Ok(Self::Sd),
            "cg" => Ok(Self::Cg),
            "rmm/diis" => Ok(Self::RmmDiis),
            other => Err(castep_cell_io::Error::Message(format!("unknown ElectronicMinimizer: {other}"))),
        }
    }
}

impl ToCell for ElectronicMinimizer {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("ELECTRONIC_MINIMIZER", self.to_cell_value())
    }
}

impl ToCellValue for ElectronicMinimizer {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                ElectronicMinimizer::Sd => "SD",
                ElectronicMinimizer::Cg => "CG",
                ElectronicMinimizer::RmmDiis => "RMM/DIIS", // If added
            }
            .to_string(),
        )
    }
}


