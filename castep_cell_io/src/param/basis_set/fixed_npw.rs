use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult};
use castep_cell_fmt::query::value_as_bool;
use serde::{Deserialize, Serialize};

/// Determines whether a fixed number of plane waves (TRUE) or a fixed
/// plane wave cutoff energy (FALSE) will be used when doing a variable cell calculation.
///
/// Keyword type: Logical
///
/// Default: false
///
/// Example:
/// FIXED_NPW : TRUE
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "FIXED_NPW")]
pub struct FixedNpw(pub bool);

impl FromCellValue for FixedNpw {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl FromKeyValue for FixedNpw {
    const KEY_NAME: &'static str = "FIXED_NPW";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for FixedNpw {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("FIXED_NPW", CellValue::Bool(self.0))
    }
}

impl ToCellValue for FixedNpw {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}


