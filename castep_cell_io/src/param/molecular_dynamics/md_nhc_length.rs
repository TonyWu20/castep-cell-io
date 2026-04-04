use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_u32;

/// Controls the length of Nosé-Hoover thermostat chains.
///
/// Keyword type: Integer
///
/// Default: 5
///
/// Example:
/// MD_NHC_LENGTH : 3
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MdNhcLength(pub u32); // Using u32 as it's a count/length

impl Default for MdNhcLength {
    fn default() -> Self {
        Self(5) // Default is 5
    }
}

impl FromCellValue for MdNhcLength {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for MdNhcLength {
    const KEY_NAME: &'static str = "MD_NHC_LENGTH";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MdNhcLength {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_NHC_LENGTH", CellValue::UInt(self.0))
    }
}

impl ToCellValue for MdNhcLength {
    fn to_cell_value(&self) -> CellValue {
        CellValue::UInt(self.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::UInt(3);
        let result = MdNhcLength::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 3);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(MdNhcLength::KEY_NAME, "MD_NHC_LENGTH");
    }
}

