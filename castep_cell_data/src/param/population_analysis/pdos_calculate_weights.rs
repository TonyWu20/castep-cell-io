use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_bool;

/// Specifies whether or not the weight of the bands in each localized orbital
/// will be calculated for partial density of states analysis.
///
/// Keyword type: Logical
///
/// Default: false
///
/// Example:
/// PDOS_CALCULATE_WEIGHTS : TRUE
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PdosCalculateWeights(pub bool);

impl FromCellValue for PdosCalculateWeights {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl FromKeyValue for PdosCalculateWeights {
    const KEY_NAME: &'static str = "PDOS_CALCULATE_WEIGHTS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for PdosCalculateWeights {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PDOS_CALCULATE_WEIGHTS", CellValue::Bool(self.0))
    }
}

impl ToCellValue for PdosCalculateWeights {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}

