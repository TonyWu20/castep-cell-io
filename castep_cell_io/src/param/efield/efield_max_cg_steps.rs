use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_u32;

/// Controls the maximum number of conjugate gradient steps during a linear response calculation.
///
/// Keyword type: Integer
///
/// Default: 20
///
/// Example:
/// EFIELD_MAX_CG_STEPS : 30
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EfieldMaxCgSteps(pub u32);

impl Default for EfieldMaxCgSteps {
    fn default() -> Self {
        Self(20)
    }
}

impl FromCellValue for EfieldMaxCgSteps {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for EfieldMaxCgSteps {
    const KEY_NAME: &'static str = "EFIELD_MAX_CG_STEPS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for EfieldMaxCgSteps {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("EFIELD_MAX_CG_STEPS", CellValue::UInt(self.0))
    }
}

impl ToCellValue for EfieldMaxCgSteps {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::UInt(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::UInt(30);
        let result = EfieldMaxCgSteps::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 30);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(EfieldMaxCgSteps::KEY_NAME, "EFIELD_MAX_CG_STEPS");
    }
}
