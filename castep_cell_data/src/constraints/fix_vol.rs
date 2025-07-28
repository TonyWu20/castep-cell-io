use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls whether or not the volume of the cell remains fixed
/// during relaxation or molecular dynamics. Cell angles and cell lengths
/// may still be varied.
///
/// Keyword type: Logical
///
/// Default: FALSE
///
/// Example:
/// FIX_VOL : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "FIX_VOL")] // Ensures correct key name during serde
pub struct FixVOL(
    /// The logical value (true = fixed, false = not fixed).
    pub bool,
);

// Implement ToCell for FixVol to enable serialization via your custom backend
impl ToCell for FixVOL {
    fn to_cell(&self) -> Cell {
        // Create a KeyValue Cell with the name "FIX_VOL" and the boolean value
        Cell::KeyValue("FIX_VOL", CellValue::Bool(self.0))
    }
}

// Implement ToCellValue for FixVol.
impl ToCellValue for FixVOL {
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
    fn test_fix_vol_serde() {
        // 1. Test Deserialization from TRUE
        let fix_vol_true_str = "FIX_VOL : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithFixVolTrue {
            fix_vol: FixVOL,
        }
        let cell_file_true_result: Result<CellFileWithFixVolTrue, _> = from_str(fix_vol_true_str);
        assert!(
            cell_file_true_result.is_ok(),
            "Deserialization (TRUE) failed: {:?}",
            cell_file_true_result.err()
        );
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.fix_vol.0); // Clippy suggestion

        // 2. Test Deserialization from FALSE
        let fix_vol_false_str = "FIX_VOL : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithFixVolFalse {
            fix_vol: FixVOL,
        }
        let cell_file_false_result: Result<CellFileWithFixVolFalse, _> =
            from_str(fix_vol_false_str);
        assert!(
            cell_file_false_result.is_ok(),
            "Deserialization (FALSE) failed: {:?}",
            cell_file_false_result.err()
        );
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.fix_vol.0); // Clippy suggestion

        // 3. Test Serialization using ToCell
        let fix_vol_instance = FixVOL(true);
        let serialized_result = to_string(&fix_vol_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        println!("Serialized FIX_VOL (TRUE): {serialized_string}"); // Clippy suggestion
        // Basic check
        assert!(serialized_string.contains("FIX_VOL"));
        assert!(serialized_string.contains("true") || serialized_string.contains("TRUE")); // Format might vary slightly

        // Test serialization of FALSE
        let fix_vol_false_instance = FixVOL(false);
        let serialized_result_false = to_string(&fix_vol_false_instance.to_cell());
        assert!(
            serialized_result_false.is_ok(),
            "Serialization (FALSE) failed: {:?}",
            serialized_result_false.err()
        );
        let serialized_string_false = serialized_result_false.unwrap();
        println!("Serialized FIX_VOL (FALSE): {serialized_string_false}"); // Clippy suggestion
        assert!(serialized_string_false.contains("FIX_VOL"));
        assert!(
            serialized_string_false.contains("false") || serialized_string_false.contains("FALSE")
        );
    }
}
