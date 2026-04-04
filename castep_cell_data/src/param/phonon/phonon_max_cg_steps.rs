use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_u32;
use serde::{Deserialize, Serialize};

/// Controls the maximum number of conjugate gradient steps during a phonon calculation.
///
/// Keyword type: Integer
///
/// Default: 20
///
/// Example:
/// PHONON_MAX_CG_STEPS : 10
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "PHONON_MAX_CG_STEPS")]
pub struct PhononMaxCgSteps(pub u32); // Using i32

impl FromCellValue for PhononMaxCgSteps {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for PhononMaxCgSteps {
    const KEY_NAME: &'static str = "PHONON_MAX_CG_STEPS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl Default for PhononMaxCgSteps {
    fn default() -> Self {
        Self(20) // Default is 20
    }
}

impl ToCell for PhononMaxCgSteps {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PHONON_MAX_CG_STEPS", CellValue::UInt(self.0))
    }
}

impl ToCellValue for PhononMaxCgSteps {
    fn to_cell_value(&self) -> CellValue {
        CellValue::UInt(self.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::UInt(10);
        let result = PhononMaxCgSteps::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 10);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(PhononMaxCgSteps::KEY_NAME, "PHONON_MAX_CG_STEPS");
    }
}

