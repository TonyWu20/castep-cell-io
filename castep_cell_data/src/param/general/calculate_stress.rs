use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls whether or not a stress calculation will be performed.
///
/// Keyword type: Logical
///
/// Default: FALSE
///
/// Example:
/// CALCULATE_STRESS : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "CALCULATE_STRESS")]
pub struct CalculateStress(pub bool);

impl ToCell for CalculateStress {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("CALCULATE_STRESS", CellValue::Bool(self.0))
    }
}

impl ToCellValue for CalculateStress {
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
    fn test_calculate_stress_serde() {
        // 1. Test Deserialization TRUE
        let calculate_stress_true_str = "CALCULATE_STRESS : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithStressTrue {
            calculate_stress: CalculateStress,
        }

        let cell_file_true_result: Result<CellFileWithStressTrue, _> =
            from_str(calculate_stress_true_str);
        assert!(
            cell_file_true_result.is_ok(),
            "Deserialization (TRUE) failed: {:?}",
            cell_file_true_result.err()
        );
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.calculate_stress.0); // Clippy suggestion

        // 2. Test Deserialization FALSE
        let calculate_stress_false_str = "CALCULATE_STRESS : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithStressFalse {
            calculate_stress: CalculateStress,
        }

        let cell_file_false_result: Result<CellFileWithStressFalse, _> =
            from_str(calculate_stress_false_str);
        assert!(
            cell_file_false_result.is_ok(),
            "Deserialization (FALSE) failed: {:?}",
            cell_file_false_result.err()
        );
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.calculate_stress.0); // Clippy suggestion

        // 3. Test Serialization using ToCell
        let calculate_stress_instance = CalculateStress(true);
        let serialized_result = to_string(&calculate_stress_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        println!("Serialized CALCULATE_STRESS (TRUE):\n{serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("CALCULATE_STRESS"));
        assert!(serialized_string.contains("true") || serialized_string.contains("TRUE"));
    }
}
