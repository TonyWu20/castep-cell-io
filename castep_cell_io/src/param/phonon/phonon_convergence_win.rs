use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_u32;
use serde::{Deserialize, Serialize};

/// Determines the size of the convergence window during a phonon calculation.
///
/// Keyword type: Integer
///
/// Default: 2
///
/// Example:
/// PHONON_CONVERGENCE_WIN : 4
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "PHONON_CONVERGENCE_WIN")]
pub struct PhononConvergenceWin(pub u32); // Using i32

impl FromCellValue for PhononConvergenceWin {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for PhononConvergenceWin {
    const KEY_NAME: &'static str = "PHONON_CONVERGENCE_WIN";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl Default for PhononConvergenceWin {
    fn default() -> Self {
        Self(2) // Default is 2
    }
}

impl ToCell for PhononConvergenceWin {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PHONON_CONVERGENCE_WIN", CellValue::UInt(self.0))
    }
}

impl ToCellValue for PhononConvergenceWin {
    fn to_cell_value(&self) -> CellValue {
        CellValue::UInt(self.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::UInt(4);
        let result = PhononConvergenceWin::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 4);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(PhononConvergenceWin::KEY_NAME, "PHONON_CONVERGENCE_WIN");
    }
}

