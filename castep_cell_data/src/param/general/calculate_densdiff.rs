use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls whether or not a calculation of the density difference
/// with respect to the sum of atomic densities will be performed.
///
/// Keyword type: Logical
///
/// Default: FALSE
///
/// Example:
/// CALCULATE_DENSDIFF : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "CALCULATE_DENSDIFF")]
pub struct CalculateDensdiff(pub bool);

impl ToCell for CalculateDensdiff {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("CALCULATE_DENSDIFF", CellValue::Bool(self.0))
    }
}

impl ToCellValue for CalculateDensdiff {
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
    fn test_calculate_densdiff_serde() {
        // 1. Test Deserialization TRUE
        let calculate_densdiff_true_str = "CALCULATE_DENSDIFF : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithDensdiffTrue {
            calculate_densdiff: CalculateDensdiff,
        }

        let cell_file_true_result: Result<CellFileWithDensdiffTrue, _> =
            from_str(calculate_densdiff_true_str);
        assert!(
            cell_file_true_result.is_ok(),
            "Deserialization (TRUE) failed: {:?}",
            cell_file_true_result.err()
        );
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.calculate_densdiff.0); // Clippy suggestion

        // 2. Test Deserialization FALSE
        let calculate_densdiff_false_str = "CALCULATE_DENSDIFF : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithDensdiffFalse {
            calculate_densdiff: CalculateDensdiff,
        }

        let cell_file_false_result: Result<CellFileWithDensdiffFalse, _> =
            from_str(calculate_densdiff_false_str);
        assert!(
            cell_file_false_result.is_ok(),
            "Deserialization (FALSE) failed: {:?}",
            cell_file_false_result.err()
        );
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.calculate_densdiff.0); // Clippy suggestion

        // 3. Test Serialization using ToCell
        let calculate_densdiff_instance = CalculateDensdiff(false);
        let serialized_result = to_string(&calculate_densdiff_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        println!("Serialized CALCULATE_DENSDIFF (FALSE):\n{serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("CALCULATE_DENSDIFF"));
        assert!(serialized_string.contains("false") || serialized_string.contains("FALSE"));
    }
}
