use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromKeyValue, CResult};
use castep_cell_io::query::value_as_i32;

/// Specifies the number of SCF cycles between updates to the wavefunction and density data file.
///
/// Keyword type: Integer
///
/// Default: 5
///
/// Example:
/// NUM_DUMP_CYCLES : 10
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumDumpCycles(pub i32);

impl Default for NumDumpCycles {
    fn default() -> Self {
        Self(5)
    }
}

impl FromKeyValue for NumDumpCycles {
    const KEY_NAME: &'static str = "NUM_DUMP_CYCLES";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_i32(value)?))
    }
}

impl ToCell for NumDumpCycles {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("NUM_DUMP_CYCLES", CellValue::Int(self.0))
    }
}

impl ToCellValue for NumDumpCycles {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}


