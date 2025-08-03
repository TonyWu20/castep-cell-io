use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the size of the convergence window for electronic minimization during MD.
///
/// Keyword type: Integer
///
/// Default: Same as ELEC_CONVERGENCE_WIN
///
/// Example:
/// MD_ELEC_CONVERGENCE_WIN : 4
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MD_ELEC_CONVERGENCE_WIN")]
pub struct MdElecConvergenceWin(pub u32); // Using u32 as it's a window size

// Note: Default logic is context-dependent (same as ELEC_CONVERGENCE_WIN).
// The `Default` implementation here is omitted as it's not directly applicable.
// A containing struct or the application logic would need to handle this.

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
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_md_elec_convergence_win_serde() {
        let md_elec_convergence_win_str = "MD_ELEC_CONVERGENCE_WIN : 4";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMdElecConvergenceWin {
            md_elec_convergence_win: MdElecConvergenceWin,
        }

        let cell_file_result: Result<CellFileWithMdElecConvergenceWin, _> =
            from_str(md_elec_convergence_win_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.md_elec_convergence_win.0, 4);

        let md_elec_convergence_win_instance = MdElecConvergenceWin(6);
        let serialized_result = to_string(&md_elec_convergence_win_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MD_ELEC_CONVERGENCE_WIN (6): {serialized_string}");
        assert!(serialized_string.contains("MD_ELEC_CONVERGENCE_WIN"));
        assert!(serialized_string.contains("6"));
    }
}
