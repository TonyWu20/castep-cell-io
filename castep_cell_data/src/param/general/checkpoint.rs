use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_io::parse::FromKeyValue;
use castep_cell_io::query::value_as_str;

/// Specifies the name of file to which checkpoint (continuation) data are to be written.
///
/// Keyword type: String
///
/// Default: seedname.check
///
/// Example:
/// CHECKPOINT : test.check
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checkpoint(pub String);

impl FromKeyValue for Checkpoint {
    const KEY_NAME: &'static str = "CHECKPOINT";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_str(value)?.to_string()))
    }
}

impl ToCell for Checkpoint {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("CHECKPOINT", CellValue::String(self.0.clone()))
    }
}

impl ToCellValue for Checkpoint {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(self.0.clone())
    }
}


