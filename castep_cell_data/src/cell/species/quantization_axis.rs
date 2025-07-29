use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Defines the quantization (magnetization) axis for LDA+U calculations.
///
/// Keyword type: Real Vector
///
/// Default: The fractional coordinates equivalent to the c axis.
///
/// Example:
/// QUANTIZATION_AXIS : 1 1 -1
/// This input defines the quantization axis (1 1 -1) which is most appropriate
/// for a fcc structure (along one of the fourth order axes).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "QUANTIZATION_AXIS")] // Ensures correct key name during serde
#[serde(transparent)]
pub struct QuantizationAxis {
    /// The fractional coordinates [fa, fb, fc] defining the quantization axis.
    pub direction: [f64; 3],
}

// Implement ToCell for QuantizationAxis to enable serialization via your custom backend
impl ToCell for QuantizationAxis {
    fn to_cell(&self) -> Cell {
        // Create a KeyValue Cell with the name "QUANTIZATION_AXIS" and an Array value
        // containing the three float components.
        Cell::KeyValue(
            "QUANTIZATION_AXIS",
            CellValue::Array(
                self.direction
                    .iter()
                    .map(|&val| CellValue::Float(val))
                    .collect(),
            ),
        )
    }
}

// Implement ToCellValue for QuantizationAxis.
impl ToCellValue for QuantizationAxis {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(
            self.direction
                .iter()
                .map(|&val| CellValue::Float(val))
                .collect(),
        )
    }
}

// If direct deserialization from "QUANTIZATION_AXIS : fa fb fc" into [f64; 3]
// doesn't work seamlessly with your parser, an intermediate representation
// and `From` implementation might be needed, similar to other vector keywords.
// For now, deriving Deserialize is assumed to work.

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_quantization_axis_serde() {
        // 1. Test Deserialization (assuming parser provides correct structure)
        // Note: This test's success depends on how `castep_cell_serde` parses
        // "QUANTIZATION_AXIS : 1 1 -1". It needs to provide a structure
        // that can be deserialized into `QuantizationAxis`.
        let quantization_axis_str = "QUANTIZATION_AXIS : 1.0 1.0 -1.0";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithQuantizationAxis {
            quantization_axis: QuantizationAxis,
        }

        let cell_file_result: Result<CellFileWithQuantizationAxis, _> =
            from_str(quantization_axis_str);
        // Assert successful deserialization (assuming the parser handles it)
        // If this fails, an intermediate repr + From implementation is needed.
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.quantization_axis.direction, [1.0, 1.0, -1.0]);

        // 2. Test Serialization using ToCell
        let quantization_axis_instance = QuantizationAxis {
            direction: [0.0, 0.0, 1.0], // Default-like example (c-axis)
        };
        let serialized_result = to_string(&quantization_axis_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        println!("Serialized QUANTIZATION_AXIS (0 0 1):\n{serialized_string}"); // Clippy suggestion
        // Basic checks
        assert!(serialized_string.contains("QUANTIZATION_AXIS"));
        assert!(serialized_string.contains("0")); // Checks for 0.0 values
        assert!(serialized_string.contains("1")); // Checks for 1.0 value
        // Note: Exact string format might depend on float formatting in `to_string`
    }
}
