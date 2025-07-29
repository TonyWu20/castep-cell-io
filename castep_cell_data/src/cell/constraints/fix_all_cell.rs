use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls whether or not all of the lattice parameters remain fixed
/// during relaxation or molecular dynamics.
///
/// Keyword type: Logical
///
/// Default: FALSE
///
/// Example:
/// FIX_ALL_CELL : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "FIX_ALL_CELL")] // Ensures correct key name during serde
pub struct FixAllCell(
    /// The logical value (true = fixed, false = not fixed).
    pub bool,
);

// Implement ToCell for FixAllCell to enable serialization via your custom backend
impl ToCell for FixAllCell {
    fn to_cell(&self) -> Cell {
        // Create a KeyValue Cell with the name "FIX_ALL_CELL" and the boolean value
        Cell::KeyValue("FIX_ALL_CELL", CellValue::Bool(self.0))
    }
}

// Implement ToCellValue for FixAllCell.
// While ToCell is the primary trait for top-level serialization,
// ToCellValue can be useful if this type needs to be nested or used
// in a context expecting a CellValue. It's also consistent with other types.
impl ToCellValue for FixAllCell {
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
    fn test_fix_all_cell_serde() {
        // 1. Test Deserialization from TRUE
        let fix_all_cell_true_str = "FIX_ALL_CELL : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithFixTrue {
            fix_all_cell: FixAllCell,
        }
        let cell_file_true_result: Result<CellFileWithFixTrue, _> = from_str(fix_all_cell_true_str);
        assert!(
            cell_file_true_result.is_ok(),
            "Deserialization (TRUE) failed: {:?}",
            cell_file_true_result.err()
        );
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.fix_all_cell.0);

        // 2. Test Deserialization from FALSE
        let fix_all_cell_false_str = "FIX_ALL_CELL : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithFixFalse {
            fix_all_cell: FixAllCell,
        }
        let cell_file_false_result: Result<CellFileWithFixFalse, _> =
            from_str(fix_all_cell_false_str);
        assert!(
            cell_file_false_result.is_ok(),
            "Deserialization (FALSE) failed: {:?}",
            cell_file_false_result.err()
        );
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.fix_all_cell.0);

        // 3. Test Serialization using ToCell
        let fix_all_cell_instance = FixAllCell(true);
        let serialized_result = to_string(&fix_all_cell_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        println!("Serialized FIX_ALL_CELL (TRUE): {serialized_string}");
        // Basic check
        assert!(serialized_string.contains("FIX_ALL_CELL"));
        assert!(serialized_string.contains("true") || serialized_string.contains("TRUE")); // Format might vary slightly

        // Test serialization of FALSE
        let fix_all_cell_false_instance = FixAllCell(false);
        let serialized_result_false = to_string(&fix_all_cell_false_instance.to_cell());
        assert!(
            serialized_result_false.is_ok(),
            "Serialization (FALSE) failed: {:?}",
            serialized_result_false.err()
        );
        let serialized_string_false = serialized_result_false.unwrap();
        println!("Serialized FIX_ALL_CELL (FALSE): {serialized_string_false}");
        assert!(serialized_string_false.contains("FIX_ALL_CELL"));
        assert!(
            serialized_string_false.contains("false") || serialized_string_false.contains("FALSE")
        );
    }
}
