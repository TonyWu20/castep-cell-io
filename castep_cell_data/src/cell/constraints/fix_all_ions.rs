use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromCellValue, FromKeyValue, CResult, Error};

/// Controls whether or not all of the ionic positions remain fixed
/// during relaxation or molecular dynamics.
///
/// Keyword type: Logical
///
/// Default: FALSE
///
/// Example:
/// FIX_ALL_IONS : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixAllIons(
    /// The logical value (true = fixed, false = not fixed).
    pub bool,
);

impl FromCellValue for FixAllIons {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Bool(b) => Ok(FixAllIons(*b)),
            _ => Err(Error::Message(
                "FixAllIons must be a boolean".into(),
            )),
        }
    }
}

impl FromKeyValue for FixAllIons {
    const KEY_NAME: &'static str = "FIX_ALL_IONS";
    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for FixAllIons {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("FIX_ALL_IONS", CellValue::Bool(self.0))
    }
}

impl ToCellValue for FixAllIons {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}


