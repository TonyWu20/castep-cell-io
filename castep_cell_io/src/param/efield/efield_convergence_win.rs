use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_u32;

/// Determines the size of the convergence window during a linear response calculation.
///
/// Keyword type: Integer
///
/// Default: 2
///
/// Example:
/// EFIELD_CONVERGENCE_WIN : 30
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EfieldConvergenceWin(pub u32);

impl Default for EfieldConvergenceWin {
    fn default() -> Self {
        Self(2)
    }
}

impl FromCellValue for EfieldConvergenceWin {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for EfieldConvergenceWin {
    const KEY_NAME: &'static str = "EFIELD_CONVERGENCE_WIN";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for EfieldConvergenceWin {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("EFIELD_CONVERGENCE_WIN", CellValue::UInt(self.0))
    }
}

impl ToCellValue for EfieldConvergenceWin {
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
        let result = EfieldConvergenceWin::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 30);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(EfieldConvergenceWin::KEY_NAME, "EFIELD_CONVERGENCE_WIN");
    }
}
