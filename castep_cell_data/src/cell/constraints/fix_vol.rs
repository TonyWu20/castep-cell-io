use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromCellValue, FromKeyValue, CResult, Error};

/// Controls whether or not the volume of the cell remains fixed
/// during relaxation or molecular dynamics. Cell angles and cell lengths
/// may still be varied.
///
/// Keyword type: Logical
///
/// Default: FALSE
///
/// Example:
/// FIX_VOL : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixVOL(
    /// The logical value (true = fixed, false = not fixed).
    pub bool,
);

impl FromCellValue for FixVOL {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Bool(b) => Ok(FixVOL(*b)),
            _ => Err(Error::Message(
                "FixVOL must be a boolean".into(),
            )),
        }
    }
}

impl FromKeyValue for FixVOL {
    const KEY_NAME: &'static str = "FIX_VOL";
    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for FixVOL {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("FIX_VOL", CellValue::Bool(self.0))
    }
}

impl ToCellValue for FixVOL {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}


