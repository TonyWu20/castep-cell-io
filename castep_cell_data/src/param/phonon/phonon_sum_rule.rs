use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_bool;
use serde::{Deserialize, Serialize};

/// Specifies whether to correct the dynamical matrix to enforce the acoustic sum rule.
///
/// Keyword type: Logical
///
/// Default: false
///
/// Example:
/// PHONON_SUM_RULE : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename = "PHONON_SUM_RULE")]
pub struct PhononSumRule(pub bool);

impl FromCellValue for PhononSumRule {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl FromKeyValue for PhononSumRule {
    const KEY_NAME: &'static str = "PHONON_SUM_RULE";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for PhononSumRule {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PHONON_SUM_RULE", CellValue::Bool(self.0))
    }
}

impl ToCellValue for PhononSumRule {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}


