use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_fmt::parse::FromKeyValue;
use castep_cell_fmt::query::value_as_str;

/// Specifies the model file used to continue the job.
///
/// Keyword type: String
///
/// Default: NULL (no continuation)
///
/// Example:
/// CONTINUATION : DEFAULT
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Continuation(pub String); // Could potentially be an enum for NULL/DEFAULT/values

impl FromKeyValue for Continuation {
    const KEY_NAME: &'static str = "CONTINUATION";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_str(value)?.to_string()))
    }
}

impl ToCell for Continuation {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("CONTINUATION", CellValue::String(self.0.clone()))
    }
}

impl ToCellValue for Continuation {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::String(self.0.clone())
    }
}


