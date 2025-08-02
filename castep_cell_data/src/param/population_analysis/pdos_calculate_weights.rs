use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies whether or not the weight of the bands in each localized orbital
/// will be calculated for partial density of states analysis.
///
/// Keyword type: Logical
///
/// Default: false
///
/// Example:
/// PDOS_CALCULATE_WEIGHTS : TRUE
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "PDOS_CALCULATE_WEIGHTS")]
pub struct PdosCalculateWeights(pub bool);

impl ToCell for PdosCalculateWeights {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PDOS_CALCULATE_WEIGHTS", CellValue::Bool(self.0))
    }
}

impl ToCellValue for PdosCalculateWeights {
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
    fn test_pdos_calculate_weights_serde() {
        // 1. Test Deserialization TRUE
        let pdos_calc_weights_true_str = "PDOS_CALCULATE_WEIGHTS : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPdosWeightsTrue {
            pdos_calculate_weights: PdosCalculateWeights,
        }

        let cell_file_true_result: Result<CellFileWithPdosWeightsTrue, _> =
            from_str(pdos_calc_weights_true_str);
        assert!(
            cell_file_true_result.is_ok(),
            "Deserialization (TRUE) failed: {:?}",
            cell_file_true_result.err()
        );
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.pdos_calculate_weights.0);

        // 2. Test Deserialization FALSE
        let pdos_calc_weights_false_str = "PDOS_CALCULATE_WEIGHTS : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPdosWeightsFalse {
            pdos_calculate_weights: PdosCalculateWeights,
        }

        let cell_file_false_result: Result<CellFileWithPdosWeightsFalse, _> =
            from_str(pdos_calc_weights_false_str);
        assert!(
            cell_file_false_result.is_ok(),
            "Deserialization (FALSE) failed: {:?}",
            cell_file_false_result.err()
        );
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.pdos_calculate_weights.0);

        // 3. Test Serialization using ToCell
        let pdos_calc_weights_instance = PdosCalculateWeights(true);
        let serialized_result = to_string(&pdos_calc_weights_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized PDOS_CALCULATE_WEIGHTS (TRUE): {serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("PDOS_CALCULATE_WEIGHTS"));
        assert!(serialized_string.contains("true") || serialized_string.contains("TRUE"));

        // 4. Test Default
        assert_eq!(PdosCalculateWeights::default(), PdosCalculateWeights(false));
    }
}
