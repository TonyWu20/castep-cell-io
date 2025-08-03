use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the size of the convergence window during a linear response calculation.
///
/// Keyword type: Integer
///
/// Default: 2
///
/// Example:
/// EFIELD_CONVERGENCE_WIN : 30
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "EFIELD_CONVERGENCE_WIN")]
pub struct EfieldConvergenceWin(pub u32); // Using u32 as it's a window size

impl Default for EfieldConvergenceWin {
    fn default() -> Self {
        Self(2) // Default is 2
    }
}

impl ToCell for EfieldConvergenceWin {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("EFIELD_CONVERGENCE_WIN", CellValue::UInt(self.0))
    }
}

impl ToCellValue for EfieldConvergenceWin {
    fn to_cell_value(&self) -> CellValue {
        CellValue::UInt(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_efield_convergence_win_serde() {
        let efield_convergence_win_str = "EFIELD_CONVERGENCE_WIN : 30";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithEfieldConvergenceWin {
            efield_convergence_win: EfieldConvergenceWin,
        }

        let cell_file_result: Result<CellFileWithEfieldConvergenceWin, _> =
            from_str(efield_convergence_win_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.efield_convergence_win.0, 30);

        let efield_convergence_win_instance = EfieldConvergenceWin(15);
        let serialized_result = to_string(&efield_convergence_win_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized EFIELD_CONVERGENCE_WIN (15): {serialized_string}");
        assert!(serialized_string.contains("EFIELD_CONVERGENCE_WIN"));
        assert!(serialized_string.contains("15"));

        assert_eq!(EfieldConvergenceWin::default(), EfieldConvergenceWin(2));
    }
}
