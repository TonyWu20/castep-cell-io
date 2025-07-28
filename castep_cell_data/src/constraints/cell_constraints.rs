use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Represents the constraints on lattice parameters (lengths and angles) during relaxation/MD.
///
/// Consists of two sets of indices:
/// - One for the lattice vector magnitudes (a, b, c)
/// - One for the lattice angles (alpha, beta, gamma)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "CELL_CONSTRAINTS")] // Ensure correct block name during serde
pub struct CellConstraints {
    /// Constraints for lattice vector magnitudes [a, b, c].
    /// 0 = fixed, positive integer = free to vary, shared integers = linked values.
    pub lengths: [u32; 3],
    /// Constraints for lattice angles [alpha, beta, gamma] in degrees.
    /// 0 = fixed, positive integer = free to vary, shared integers = linked values.
    pub angles: [u32; 3],
}

impl ToCell for CellConstraints {
    /// Converts the struct directly into the intermediate `Cell` representation
    /// for the CELL_CONSTRAINTS block serialization.
    fn to_cell(&self) -> Cell {
        // Use functional style and iterator methods
        let block_content = [
            CellValue::Array(self.lengths.iter().map(|&v| CellValue::UInt(v)).collect()),
            CellValue::Array(self.angles.iter().map(|&v| CellValue::UInt(v)).collect()),
        ]
        .to_vec(); // Convert array to Vec

        Cell::Block("CELL_CONSTRAINTS", block_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_cell_constraints_serde() {
        // Define a test struct matching the expected .cell file structure
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileConstraints {
            cell_constraints: CellConstraints,
        }

        // Test string based on the documentation example
        let cell_constraints_str = r#"%BLOCK CELL_CONSTRAINTS
       1       2       3
       0       0       0
%ENDBLOCK CELL_CONSTRAINTS
"#;

        // 1. Test Deserialization
        let cell_file_result: Result<CellFileConstraints, _> = from_str(cell_constraints_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();

        // Debug print the deserialized struct
        println!(
            "Deserialized CELL_CONSTRAINTS: {:#?}",
            cell_file.cell_constraints
        );

        // Verify the content
        assert_eq!(cell_file.cell_constraints.lengths, [1, 2, 3]);
        assert_eq!(cell_file.cell_constraints.angles, [0, 0, 0]);

        // 2. Test Serialization using ToCell
        let test_constraints = CellConstraints {
            lengths: [1, 1, 2], // e.g., a=b, c independent
            angles: [0, 0, 3],  // e.g., alpha=beta=90 (fixed), gamma independent
        };

        let serialized_result = to_string(&test_constraints.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        // Print the serialized string
        println!("Serialized CELL_CONSTRAINTS:\n{serialized_string}"); // Clippy suggestion

        // Basic checks on the serialized string
        assert!(serialized_string.contains("%BLOCK CELL_CONSTRAINTS"));
        assert!(serialized_string.contains("%ENDBLOCK CELL_CONSTRAINTS"));
        assert!(serialized_string.contains("1")); // From lengths [1, 1, 2]
        assert!(serialized_string.contains("0")); // From angles [0, 0, 3]
        // Note: Exact string comparison can be brittle due to formatting/whitespace/order
    }
}
