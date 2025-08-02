use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies whether or not to perform a population analysis on the final ground state.
///
/// Keyword type: Logical
///
/// Default: true
///
/// Example:
/// POPN_CALCULATE : FALSE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "POPN_CALCULATE")]
pub struct PopnCalculate(pub bool);

impl Default for PopnCalculate {
    fn default() -> Self {
        Self(true) // Default is TRUE
    }
}

impl ToCell for PopnCalculate {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("POPN_CALCULATE", CellValue::Bool(self.0))
    }
}

impl ToCellValue for PopnCalculate {
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
    fn test_popn_calculate_serde() {
        // 1. Test Deserialization TRUE
        let popn_calculate_true_str = "POPN_CALCULATE : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPopnCalculateTrue {
            popn_calculate: PopnCalculate,
        }

        let cell_file_true_result: Result<CellFileWithPopnCalculateTrue, _> =
            from_str(popn_calculate_true_str);
        assert!(
            cell_file_true_result.is_ok(),
            "Deserialization (TRUE) failed: {:?}",
            cell_file_true_result.err()
        );
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.popn_calculate.0);

        // 2. Test Deserialization FALSE
        let popn_calculate_false_str = "POPN_CALCULATE : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPopnCalculateFalse {
            popn_calculate: PopnCalculate,
        }

        let cell_file_false_result: Result<CellFileWithPopnCalculateFalse, _> =
            from_str(popn_calculate_false_str);
        assert!(
            cell_file_false_result.is_ok(),
            "Deserialization (FALSE) failed: {:?}",
            cell_file_false_result.err()
        );
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.popn_calculate.0);

        // 3. Test Serialization using ToCell
        let popn_calculate_instance = PopnCalculate(false);
        let serialized_result = to_string(&popn_calculate_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized POPN_CALCULATE (FALSE): {serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("POPN_CALCULATE"));
        assert!(serialized_string.contains("false") || serialized_string.contains("FALSE"));

        // 4. Test Default
        assert_eq!(PopnCalculate::default(), PopnCalculate(true));
    }
}
