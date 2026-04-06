use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_u32;

/// Determines the total number of steps for a molecular dynamics calculation.
///
/// Keyword type: Integer
///
/// Default: 100
///
/// Example:
/// MD_NUM_ITER : 125
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MdNumIter(pub u32); // Using u32 as it's a count of iterations

impl Default for MdNumIter {
    fn default() -> Self {
        Self(100) // Default is 100
    }
}

impl FromCellValue for MdNumIter {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for MdNumIter {
    const KEY_NAME: &'static str = "MD_NUM_ITER";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MdNumIter {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("MD_NUM_ITER", CellValue::UInt(self.0))
    }
}

impl ToCellValue for MdNumIter {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::UInt(self.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::UInt(125);
        let result = MdNumIter::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 125);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(MdNumIter::KEY_NAME, "MD_NUM_ITER");
    }
}

