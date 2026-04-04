use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult};
use castep_cell_fmt::query::value_as_i32;
use serde::{Deserialize, Serialize};

/// Determines the maximum number of densities to store in the history used during the density mixing procedure.
///
/// Keyword type: Integer
///
/// Default: 7
///
/// Example:
/// MIX_HISTORY_LENGTH : 5
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MIX_HISTORY_LENGTH")]
pub struct MixHistoryLength(pub i32);

impl Default for MixHistoryLength {
    fn default() -> Self {
        Self(7)
    }
}

impl FromCellValue for MixHistoryLength {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_i32(value)?))
    }
}

impl FromKeyValue for MixHistoryLength {
    const KEY_NAME: &'static str = "MIX_HISTORY_LENGTH";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MixHistoryLength {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MIX_HISTORY_LENGTH", CellValue::Int(self.0))
    }
}

impl ToCellValue for MixHistoryLength {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}

