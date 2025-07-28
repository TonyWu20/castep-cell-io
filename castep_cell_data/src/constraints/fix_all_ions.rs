use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls whether or not all of the ionic positions remain fixed
/// during relaxation or molecular dynamics.
///
/// Keyword type: Logical
///
/// Default: FALSE
///
/// Example:
/// FIX_ALL_IONS : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "FIX_ALL_IONS")] // Ensures correct key name during serde
pub struct FixAllIons(
    /// The logical value (true = fixed, false = not fixed).
    pub bool,
);

// Implement ToCell for FixAllIons to enable serialization via your custom backend
impl ToCell for FixAllIons {
    fn to_cell(&self) -> Cell {
        // Create a KeyValue Cell with the name "FIX_ALL_IONS" and the boolean value
        Cell::KeyValue("FIX_ALL_IONS", CellValue::Bool(self.0))
    }
}

// Implement ToCellValue for FixAllIons.
impl ToCellValue for FixAllIons {
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
    fn test_fix_all_ions_serde() {
        // 1. Test Deserialization from TRUE
        let fix_all_ions_true_str = "FIX_ALL_IONS : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithFixIonsTrue {
            fix_all_ions: FixAllIons,
        }
        let cell_file_true_result: Result<CellFileWithFixIonsTrue, _> =
            from_str(fix_all_ions_true_str);
        assert!(
            cell_file_true_result.is_ok(),
            "Deserialization (TRUE) failed: {:?}",
            cell_file_true_result.err()
        );
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.fix_all_ions.0); // Clippy suggestion

        // 2. Test Deserialization from FALSE
        let fix_all_ions_false_str = "FIX_ALL_IONS : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithFixIonsFalse {
            fix_all_ions: FixAllIons,
        }
        let cell_file_false_result: Result<CellFileWithFixIonsFalse, _> =
            from_str(fix_all_ions_false_str);
        assert!(
            cell_file_false_result.is_ok(),
            "Deserialization (FALSE) failed: {:?}",
            cell_file_false_result.err()
        );
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.fix_all_ions.0); // Clippy suggestion

        // 3. Test Serialization using ToCell
        let fix_all_ions_instance = FixAllIons(true);
        let serialized_result = to_string(&fix_all_ions_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        println!("Serialized FIX_ALL_IONS (TRUE): {serialized_string}"); // Clippy suggestion
        // Basic check
        assert!(serialized_string.contains("FIX_ALL_IONS"));
        assert!(serialized_string.contains("true") || serialized_string.contains("TRUE")); // Format might vary slightly

        // Test serialization of FALSE
        let fix_all_ions_false_instance = FixAllIons(false);
        let serialized_result_false = to_string(&fix_all_ions_false_instance.to_cell());
        assert!(
            serialized_result_false.is_ok(),
            "Serialization (FALSE) failed: {:?}",
            serialized_result_false.err()
        );
        let serialized_string_false = serialized_result_false.unwrap();
        println!("Serialized FIX_ALL_IONS (FALSE): {serialized_string_false}"); // Clippy suggestion
        assert!(serialized_string_false.contains("FIX_ALL_IONS"));
        assert!(
            serialized_string_false.contains("false") || serialized_string_false.contains("FALSE")
        );
    }
}
