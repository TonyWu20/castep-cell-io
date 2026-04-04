use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_io::parse::FromKeyValue;
use castep_cell_io::query::value_as_i32;

/// Controls the initialization of random number seeds.
///
/// Keyword type: Integer (expert)
///
/// Default: 0
///
/// Example:
/// RAND_SEED : -25
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RandSeed(pub i32);

impl FromKeyValue for RandSeed {
    const KEY_NAME: &'static str = "RAND_SEED";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_i32(value)?))
    }
}

impl ToCell for RandSeed {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("RAND_SEED", CellValue::Int(self.0))
    }
}

impl ToCellValue for RandSeed {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}


