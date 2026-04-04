use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_io::parse::FromKeyValue;
use castep_cell_io::query::value_as_bool;

/// Specifies whether the electron localization function (ELF) is written to an ASCII file.
///
/// Keyword type: Logical
///
/// Default: FALSE
///
/// Example:
/// WRITE_FORMATTED_ELF : TRUE
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct WriteFormattedElf(pub bool);

impl FromKeyValue for WriteFormattedElf {
    const KEY_NAME: &'static str = "WRITE_FORMATTED_ELF";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl ToCell for WriteFormattedElf {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("WRITE_FORMATTED_ELF", CellValue::Bool(self.0))
    }
}

impl ToCellValue for WriteFormattedElf {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}


