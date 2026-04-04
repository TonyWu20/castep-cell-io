use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_u32;
use serde::{Deserialize, Serialize};

/// Controls the maximum number of iterations in a phonon calculation.
///
/// Keyword type: Integer
///
/// Default: 50
///
/// Example:
/// PHONON_MAX_CYCLES : 30
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "PHONON_MAX_CYCLES")]
pub struct PhononMaxCycles(pub u32); // Using i32

impl FromCellValue for PhononMaxCycles {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for PhononMaxCycles {
    const KEY_NAME: &'static str = "PHONON_MAX_CYCLES";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl Default for PhononMaxCycles {
    fn default() -> Self {
        Self(50) // Default is 50
    }
}

impl ToCell for PhononMaxCycles {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PHONON_MAX_CYCLES", CellValue::UInt(self.0))
    }
}

impl ToCellValue for PhononMaxCycles {
    fn to_cell_value(&self) -> CellValue {
        CellValue::UInt(self.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::UInt(30);
        let result = PhononMaxCycles::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 30);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(PhononMaxCycles::KEY_NAME, "PHONON_MAX_CYCLES");
    }
}

