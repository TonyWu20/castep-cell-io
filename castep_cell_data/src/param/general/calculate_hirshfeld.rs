use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls whether or not a calculation of Hirshfeld atomic charges
/// will be performed.
///
/// Keyword type: Logical
///
/// Default: FALSE
///
/// Example:
/// CALCULATE_HIRSHFELD : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "CALCULATE_HIRSHFELD")]
pub struct CalculateHirshfeld(pub bool);

impl ToCell for CalculateHirshfeld {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("CALCULATE_HIRSHFELD", CellValue::Bool(self.0))
    }
}

impl ToCellValue for CalculateHirshfeld {
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
    fn test_calculate_hirshfeld_serde() {
        // 1. Test Deserialization TRUE
        let calculate_hirshfeld_true_str = "CALCULATE_HIRSHFELD : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithHirshfeldTrue {
            calculate_hirshfeld: CalculateHirshfeld,
        }

        let cell_file_true_result: Result<CellFileWithHirshfeldTrue, _> =
            from_str(calculate_hirshfeld_true_str);
        assert!(
            cell_file_true_result.is_ok(),
            "Deserialization (TRUE) failed: {:?}",
            cell_file_true_result.err()
        );
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.calculate_hirshfeld.0); // Clippy suggestion

        // 2. Test Deserialization FALSE
        let calculate_hirshfeld_false_str = "CALCULATE_HIRSHFELD : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithHirshfeldFalse {
            calculate_hirshfeld: CalculateHirshfeld,
        }

        let cell_file_false_result: Result<CellFileWithHirshfeldFalse, _> =
            from_str(calculate_hirshfeld_false_str);
        assert!(
            cell_file_false_result.is_ok(),
            "Deserialization (FALSE) failed: {:?}",
            cell_file_false_result.err()
        );
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.calculate_hirshfeld.0); // Clippy suggestion

        // 3. Test Serialization using ToCell
        let calculate_hirshfeld_instance = CalculateHirshfeld(false);
        let serialized_result = to_string(&calculate_hirshfeld_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        println!("Serialized CALCULATE_HIRSHFELD (FALSE):\n{serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("CALCULATE_HIRSHFELD"));
        assert!(serialized_string.contains("false") || serialized_string.contains("FALSE"));
    }
}
