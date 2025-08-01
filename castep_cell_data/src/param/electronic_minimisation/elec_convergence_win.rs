use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the size of the convergence window during a electronic minimization run.
///
/// Keyword type: Integer
///
/// Default: 3
///
/// Example:
/// ELEC_CONVERGENCE_WIN : 4
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "ELEC_CONVERGENCE_WIN")]
pub struct ElecConvergenceWin(pub i32); // Should be >= 2

impl Default for ElecConvergenceWin {
    fn default() -> Self {
        Self(3) // Default is 3
    }
}

impl ToCell for ElecConvergenceWin {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("ELEC_CONVERGENCE_WIN", CellValue::Int(self.0))
    }
}

impl ToCellValue for ElecConvergenceWin {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_elec_convergence_win_serde() {
        let elec_convergence_win_str = "ELEC_CONVERGENCE_WIN : 4";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithElecConvergenceWin {
            elec_convergence_win: ElecConvergenceWin,
        }

        let cell_file_result: Result<CellFileWithElecConvergenceWin, _> =
            from_str(elec_convergence_win_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.elec_convergence_win.0, 4);

        let elec_convergence_win_instance = ElecConvergenceWin(5);
        let serialized_result = to_string(&elec_convergence_win_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized ELEC_CONVERGENCE_WIN (5):\n{serialized_string}");
        assert!(serialized_string.contains("ELEC_CONVERGENCE_WIN"));
        assert!(serialized_string.contains("5"));

        assert_eq!(ElecConvergenceWin::default(), ElecConvergenceWin(3));
    }
}
