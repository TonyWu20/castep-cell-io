use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
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
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_phonon_convergence_win_serde() {
        let phonon_convergence_win_str = "PHONON_CONVERGENCE_WIN : 4";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPhononConvergenceWin {
            phonon_convergence_win: PhononConvergenceWin,
        }

        let cell_file_result: Result<CellFileWithPhononConvergenceWin, _> =
            from_str(phonon_convergence_win_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.phonon_convergence_win.0, 4);

        let phonon_convergence_win_instance = PhononConvergenceWin(5);
        let serialized_result = to_string(&phonon_convergence_win_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized PHONON_CONVERGENCE_WIN (5): {serialized_string}");
        assert!(serialized_string.contains("PHONON_CONVERGENCE_WIN"));
        assert!(serialized_string.contains("5"));

        assert_eq!(PhononConvergenceWin::default(), PhononConvergenceWin(2));
    }
}
