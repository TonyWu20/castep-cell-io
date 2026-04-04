use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromCellValue, FromKeyValue, CResult, Error};

/// Controls whether or not all of the lattice parameters remain fixed
/// during relaxation or molecular dynamics.
///
/// Keyword type: Logical
///
/// Default: FALSE
///
/// Example:
/// FIX_ALL_CELL : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixAllCell(
    /// The logical value (true = fixed, false = not fixed).
    pub bool,
);

impl FromCellValue for FixAllCell {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Bool(b) => Ok(FixAllCell(*b)),
            _ => Err(Error::Message(
                "FixAllCell must be a boolean".into(),
            )),
        }
    }
}

impl FromKeyValue for FixAllCell {
    const KEY_NAME: &'static str = "FIX_ALL_CELL";
    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for FixAllCell {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("FIX_ALL_CELL", CellValue::Bool(self.0))
    }
}

impl ToCellValue for FixAllCell {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}


