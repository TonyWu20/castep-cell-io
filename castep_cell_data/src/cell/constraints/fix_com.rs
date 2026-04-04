use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromCellValue, FromKeyValue, CResult, Error};

/// Controls whether or not the center of mass of the ions remains fixed
/// during relaxation or molecular dynamics.
///
/// Keyword type: Logical
///
/// Default: If FIX_ALL_IONS : FALSE then the default value is TRUE.
///
/// Example:
/// FIX_COM : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixCOM(
    /// The logical value (true = fixed, false = not fixed).
    pub bool,
);

impl FromCellValue for FixCOM {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Bool(b) => Ok(FixCOM(*b)),
            _ => Err(Error::Message(
                "FixCOM must be a boolean".into(),
            )),
        }
    }
}

impl FromKeyValue for FixCOM {
    const KEY_NAME: &'static str = "FIX_COM";
    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for FixCOM {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("FIX_COM", CellValue::Bool(self.0))
    }
}

impl ToCellValue for FixCOM {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}


