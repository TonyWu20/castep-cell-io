// File: general/spin_polarized.rs
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls whether or not different wavefunctions are used for different spins.
///
/// Keyword type: Logical
///
/// Default: false
///
/// Example:
/// SPIN_POLARIZED : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "SPIN_POLARIZED")]
pub struct SpinPolarized(pub bool);

impl Default for SpinPolarized {
    fn default() -> Self {
        Self(false) // Default is FALSE
    }
}

impl ToCell for SpinPolarized {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("SPIN_POLARIZED", CellValue::Bool(self.0))
    }
}

impl ToCellValue for SpinPolarized {
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
    fn test_spin_polarized_serde() {
        // 1. Test Deserialization TRUE
        let spin_polarized_true_str = "SPIN_POLARIZED : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithSpinTrue {
            spin_polarized: SpinPolarized,
        }

        let cell_file_true_result: Result<CellFileWithSpinTrue, _> =
            from_str(spin_polarized_true_str);
        assert!(
            cell_file_true_result.is_ok(),
            "Deserialization (TRUE) failed: {:?}",
            cell_file_true_result.err()
        );
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.spin_polarized.0); // Clippy suggestion

        // 2. Test Deserialization FALSE
        let spin_polarized_false_str = "SPIN_POLARIZED : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithSpinFalse {
            spin_polarized: SpinPolarized,
        }

        let cell_file_false_result: Result<CellFileWithSpinFalse, _> =
            from_str(spin_polarized_false_str);
        assert!(
            cell_file_false_result.is_ok(),
            "Deserialization (FALSE) failed: {:?}",
            cell_file_false_result.err()
        );
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.spin_polarized.0); // Clippy suggestion

        // 3. Test Serialization using ToCell
        let spin_polarized_instance = SpinPolarized(true);
        let serialized_result = to_string(&spin_polarized_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        println!("Serialized SPIN_POLARIZED (TRUE): {serialized_string}"); // Clippy suggestion
        // Basic check
        assert!(serialized_string.contains("SPIN_POLARIZED"));
        assert!(serialized_string.contains("true") || serialized_string.contains("TRUE"));

        // 4. Test Default
        assert_eq!(SpinPolarized::default(), SpinPolarized(false));
    }
}
