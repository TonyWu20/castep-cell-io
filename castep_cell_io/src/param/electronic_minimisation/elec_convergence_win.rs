use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromKeyValue, CResult};
use castep_cell_fmt::query::value_as_i32;

/// Determines the size of the convergence window during a electronic minimization run.
///
/// Keyword type: Integer
///
/// Default: 3
///
/// Example:
/// ELEC_CONVERGENCE_WIN : 4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ElecConvergenceWin(pub i32);

impl Default for ElecConvergenceWin {
    fn default() -> Self {
        Self(3)
    }
}

impl FromKeyValue for ElecConvergenceWin {
    const KEY_NAME: &'static str = "ELEC_CONVERGENCE_WIN";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_i32(value)?))
    }
}

impl ToCell for ElecConvergenceWin {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("ELEC_CONVERGENCE_WIN", CellValue::Int(self.0))
    }
}

impl ToCellValue for ElecConvergenceWin {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Int(self.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Int(4);
        let result = ElecConvergenceWin::from_cell_value_kv(&val).unwrap();
        assert_eq!(result.0, 4);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(ElecConvergenceWin::KEY_NAME, "ELEC_CONVERGENCE_WIN");
    }
}

