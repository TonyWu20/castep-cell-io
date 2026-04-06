use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromKeyValue, CResult};
use castep_cell_fmt::query::value_as_i32;

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
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("NUM_DUMP_CYCLES", CellValue::Int(self.0))
    }
}

impl ToCellValue for NumDumpCycles {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Int(self.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Int(10);
        let result = NumDumpCycles::from_cell_value_kv(&val).unwrap();
        assert_eq!(result.0, 10);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(NumDumpCycles::KEY_NAME, "NUM_DUMP_CYCLES");
    }
}

