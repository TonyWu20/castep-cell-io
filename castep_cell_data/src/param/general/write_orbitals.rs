use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies whether or not to write seedname.orbitals binary file in a band structure calculation.
///
/// Keyword type: Logical
///
/// Default: FALSE unless TASK is BANDSTRUCTURE.
///
/// Example:
/// WRITE_ORBITALS : TRUE
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "WRITE_ORBITALS")]
pub struct WriteOrbitals(pub bool);

// Note: The default logic ("FALSE unless TASK is BANDSTRUCTURE") is context-dependent
// and cannot be easily encoded in the struct itself without access to the TASK value.
// The `Default` implementation here provides a base default of `false`.

impl ToCell for WriteOrbitals {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("WRITE_ORBITALS", CellValue::Bool(self.0))
    }
}

impl ToCellValue for WriteOrbitals {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_write_orbitals_serde() {
        // 1. Test Deserialization TRUE
        let write_orbitals_true_str = "WRITE_ORBITALS : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithWriteOrbitalsTrue {
            write_orbitals: WriteOrbitals,
        }

        let cell_file_true_result: Result<CellFileWithWriteOrbitalsTrue, _> =
            from_str(write_orbitals_true_str);
        assert!(
            cell_file_true_result.is_ok(),
            "Deserialization (TRUE) failed: {:?}",
            cell_file_true_result.err()
        );
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.write_orbitals.0);

        // 2. Test Deserialization FALSE
        let write_orbitals_false_str = "WRITE_ORBITALS : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithWriteOrbitalsFalse {
            write_orbitals: WriteOrbitals,
        }

        let cell_file_false_result: Result<CellFileWithWriteOrbitalsFalse, _> =
            from_str(write_orbitals_false_str);
        assert!(
            cell_file_false_result.is_ok(),
            "Deserialization (FALSE) failed: {:?}",
            cell_file_false_result.err()
        );
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.write_orbitals.0);

        // 3. Test Serialization using ToCell
        let write_orbitals_instance = WriteOrbitals(true);
        let serialized_result = to_string(&write_orbitals_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized WRITE_ORBITALS (TRUE):\n{serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("WRITE_ORBITALS"));
        assert!(serialized_string.contains("true") || serialized_string.contains("TRUE"));

        // 4. Test Default (base default)
        assert_eq!(WriteOrbitals::default(), WriteOrbitals(false));
        // Note: The context-dependent default ("unless TASK is BANDSTRUCTURE")
        // is not tested here as it requires access to another keyword's value.
    }
}
