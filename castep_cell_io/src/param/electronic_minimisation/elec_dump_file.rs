use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromKeyValue, CResult};
use castep_cell_fmt::query::value_as_str;

/// Determines the name of the file into which wavefunction and density data are written.
///
/// Keyword type: String
///
/// Default: seedname.wvfn
///
/// Example:
/// ELEC_DUMP_FILE : test.wvfn
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElecDumpFile(pub String);

impl FromKeyValue for ElecDumpFile {
    const KEY_NAME: &'static str = "ELEC_DUMP_FILE";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_str(value)?.to_string()))
    }
}

impl ToCell for ElecDumpFile {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("ELEC_DUMP_FILE", CellValue::String(self.0.clone()))
    }
}

impl ToCellValue for ElecDumpFile {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(self.0.clone())
    }
}


