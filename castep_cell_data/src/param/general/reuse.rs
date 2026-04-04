use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_io::parse::FromKeyValue;
use castep_cell_io::query::value_as_str;

/// Specifies the model file from which data will be read to initialize a new calculation.
///
/// Keyword type: String
///
/// Default: NULL (no reuse)
///
/// Example:
/// REUSE : DEFAULT
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reuse(pub String); // Could potentially be an enum for NULL/DEFAULT/values

impl FromKeyValue for Reuse {
    const KEY_NAME: &'static str = "REUSE";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_str(value)?.to_string()))
    }
}

impl ToCell for Reuse {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("REUSE", CellValue::String(self.0.clone()))
    }
}

impl ToCellValue for Reuse {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(self.0.clone())
    }
}


