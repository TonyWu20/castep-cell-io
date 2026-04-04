use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_u32;

/// Determines the maximum number of bands at any k-point and spin.
///
/// Keyword type: Integer
///
/// Default: Calculated based on NELECTRONS/NUP+NDOWN and NEXTRA_BANDS/PERC_EXTRA_BANDS
///
/// Example:
/// NBANDS : 64
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Nbands(pub u32);

impl FromCellValue for Nbands {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for Nbands {
    const KEY_NAME: &'static str = "NBANDS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for Nbands {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("NBANDS", CellValue::UInt(self.0))
    }
}

impl ToCellValue for Nbands {
    fn to_cell_value(&self) -> CellValue {
        CellValue::UInt(self.0)
    }
}

