use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_u32;

/// Controls the maximum number of iterations in a linear response to electric field calculation.
///
/// Keyword type: Integer
///
/// Default: 50
///
/// Example:
/// EFIELD_MAX_CYCLES : 100
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EfieldMaxCycles(pub u32);

impl Default for EfieldMaxCycles {
    fn default() -> Self {
        Self(50)
    }
}

impl FromCellValue for EfieldMaxCycles {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for EfieldMaxCycles {
    const KEY_NAME: &'static str = "EFIELD_MAX_CYCLES";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for EfieldMaxCycles {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("EFIELD_MAX_CYCLES", CellValue::UInt(self.0))
    }
}

impl ToCellValue for EfieldMaxCycles {
    fn to_cell_value(&self) -> CellValue {
        CellValue::UInt(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::UInt(100);
        let result = EfieldMaxCycles::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 100);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(EfieldMaxCycles::KEY_NAME, "EFIELD_MAX_CYCLES");
    }
}
