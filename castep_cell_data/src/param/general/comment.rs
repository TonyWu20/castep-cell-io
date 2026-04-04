use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_io::parse::FromKeyValue;
use castep_cell_io::query::value_as_str;

/// Can contain a comment string, used to label the output.
///
/// Keyword type: String
///
/// Default: (blank)
///
/// Example:
/// COMMENT : This is a test run
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment(pub String);

impl FromKeyValue for Comment {
    const KEY_NAME: &'static str = "COMMENT";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_str(value)?.to_string()))
    }
}

impl ToCell for Comment {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("COMMENT", CellValue::String(self.0.clone()))
    }
}

impl ToCellValue for Comment {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(self.0.clone())
    }
}


