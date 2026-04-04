use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult};
use castep_cell_fmt::query::value_as_i32;
use serde::{Deserialize, Serialize};

/// Controls the number of points used to estimate the BASIS_DE_DLOGE
/// in automatic calculation of the finite basis set correction.
///
/// Keyword type: Integer
///
/// Default: 3
///
/// Example:
/// FINITE_BASIS_NPOINTS : 5
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "FINITE_BASIS_NPOINTS")]
pub struct FiniteBasisNpoints(pub i32);

impl Default for FiniteBasisNpoints {
    fn default() -> Self {
        Self(3)
    }
}

impl FromCellValue for FiniteBasisNpoints {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_i32(value)?))
    }
}

impl FromKeyValue for FiniteBasisNpoints {
    const KEY_NAME: &'static str = "FINITE_BASIS_NPOINTS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for FiniteBasisNpoints {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("FINITE_BASIS_NPOINTS", CellValue::Int(self.0))
    }
}

impl ToCellValue for FiniteBasisNpoints {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}


