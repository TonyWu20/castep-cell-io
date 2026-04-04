use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_fmt::parse::FromKeyValue;
use castep_cell_fmt::query::value_as_i32;

/// Controls the paging of wavefunctions to disk in order to save memory.
///
/// Keyword type: Integer
///
/// Default: 0
///
/// Example:
/// PAGE_WVFNS : 2
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PageWvfns(pub i32);

impl FromKeyValue for PageWvfns {
    const KEY_NAME: &'static str = "PAGE_WVFNS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_i32(value)?))
    }
}

impl ToCell for PageWvfns {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PAGE_WVFNS", CellValue::Int(self.0))
    }
}

impl ToCellValue for PageWvfns {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}


