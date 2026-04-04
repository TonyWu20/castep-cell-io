use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromKeyValue, CResult};
use castep_cell_fmt::query::value_as_str;

/// Specifies the name of the file from which wavefunction and density data should be restored.
///
/// Keyword type: String
///
/// Default: NULL (no restore)
///
/// Example:
/// ELEC_RESTORE_FILE : test.wvfn
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElecRestoreFile(pub String);

impl FromKeyValue for ElecRestoreFile {
    const KEY_NAME: &'static str = "ELEC_RESTORE_FILE";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_str(value)?.to_string()))
    }
}

impl ToCell for ElecRestoreFile {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("ELEC_RESTORE_FILE", CellValue::String(self.0.clone()))
    }
}

impl ToCellValue for ElecRestoreFile {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(self.0.clone())
    }
}


