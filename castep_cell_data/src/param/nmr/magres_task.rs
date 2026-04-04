use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Defines the type of NMR calculation to be performed.
///
/// Keyword type: String
///
/// Default: MagresTask::Shielding
///
/// Example:
/// MAGRES_TASK : NMR
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MAGRES_TASK")]
pub enum MagresTask {
    /// Performs a calculation of the NMR shielding tensor for all atoms
    #[serde(alias = "shielding", alias = "SHIELDING")]
    Shielding,
    /// Performs a calculation of the electric field gradient tensor for all atoms
    #[serde(alias = "efg", alias = "EFG")]
    Efg,
    /// Performs a calculation of both the NMR shielding tensor and the EFG tensor
    #[serde(alias = "nmr", alias = "NMR")]
    Nmr,
}

impl Default for MagresTask {
    fn default() -> Self {
        Self::Shielding // Default is Shielding
    }
}

impl ToCell for MagresTask {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MAGRES_TASK", self.to_cell_value())
    }
}

impl ToCellValue for MagresTask {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                MagresTask::Shielding => "Shielding",
                MagresTask::Efg => "EFG",
                MagresTask::Nmr => "NMR",
            }
            .to_string(),
        )
    }
}


