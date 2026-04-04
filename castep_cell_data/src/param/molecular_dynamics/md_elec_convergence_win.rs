use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_u32;

/// Determines the size of the convergence window for electronic minimization during MD.
///
/// Keyword type: Integer
///
/// Default: Same as ELEC_CONVERGENCE_WIN
///
/// Example:
/// MD_ELEC_CONVERGENCE_WIN : 4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MdElecConvergenceWin(pub u32); // Using u32 as it's a window size

impl FromCellValue for MdElecConvergenceWin {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for MdElecConvergenceWin {
    const KEY_NAME: &'static str = "MD_ELEC_CONVERGENCE_WIN";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MdElecConvergenceWin {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_ELEC_CONVERGENCE_WIN", CellValue::UInt(self.0))
    }
}

impl ToCellValue for MdElecConvergenceWin {
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
        let result = MdElecConvergenceWin::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 4);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(MdElecConvergenceWin::KEY_NAME, "MD_ELEC_CONVERGENCE_WIN");
    }
}

