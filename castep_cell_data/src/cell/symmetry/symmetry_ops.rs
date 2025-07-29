use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Represents a single symmetry operation, consisting of a 3x3 rotation matrix and a 3-element translation vector.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SymmetryOp {
    /// The 3x3 rotation matrix (stored as 3 rows of 3 elements each).
    pub rotation: [[f64; 3]; 3],
    /// The 3-element translation vector.
    pub translation: [f64; 3],
}

// Implement ToCellValue for SymmetryOp if needed for nested serialization,
// though ToCell on SymmetryOps is the primary path.
// For simplicity here, we rely on the ToCell implementation for the main block.

/// Intermediate representation for deserializing the raw lines of the SYMMETRY_OPS block.
/// This struct captures the flat sequence of 3-element arrays provided by the parser.
#[derive(Debug, Deserialize)]
#[serde(transparent)]
struct SymmetryOpsRawRepr {
    /// A flat list of 3-element arrays, where every 4 arrays represent one symmetry operation:
    /// 3 for the rotation matrix rows, 1 for the translation vector.
    pub matrix_lines: Vec<[f64; 3]>,
}

/// Represents the SYMMETRY_OPS block.
///
/// Contains a list of symmetry operations under which the unit cell is invariant.
/// Each operation consists of a 3x3 rotation matrix and a 3-element translation vector.
/// Format:
/// %BLOCK SYMMETRY_OPS
/// R11 R21 R31  <- Row 1 of rotation matrix for op 1
/// R12 R22 R32  <- Row 2 of rotation matrix for op 1
/// R13 R23 R33  <- Row 3 of rotation matrix for op 1
/// T1  T2  T3   <- Translation vector for op 1
/// R11 R21 R31  <- Row 1 of rotation matrix for op 2
/// ...          <- And so on
/// %ENDBLOCK SYMMETRY_OPS
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "SymmetryOpsRawRepr")] // Use intermediate repr for deserialization
#[serde(rename = "SYMMETRY_OPS")] // Ensure correct block name during serde
pub struct SymmetryOps {
    /// The list of symmetry operations.
    pub ops: Vec<SymmetryOp>,
}

impl From<SymmetryOpsRawRepr> for SymmetryOps {
    /// Converts the flat list of 3-element arrays into structured SymmetryOp entries.
    /// Groups every 4 lines (3 for rotation, 1 for translation) into one SymmetryOp.
    fn from(raw: SymmetryOpsRawRepr) -> Self {
        // Use chunks_exact and iterator methods for a functional approach
        Self {
            ops: raw
                .matrix_lines
                .chunks_exact(4)
                .map(|chunk| SymmetryOp {
                    rotation: [chunk[0], chunk[1], chunk[2]],
                    translation: chunk[3],
                })
                .collect(),
        }
    }
}

impl ToCell for SymmetryOps {
    /// Converts the block into the intermediate `Cell` representation for serialization.
    fn to_cell(&self) -> Cell {
        let block_content = self
            .ops
            .iter()
            .flat_map(|op| {
                // Create CellValues for the rotation matrix rows and translation vector
                op.rotation
                    .iter()
                    .map(|row| {
                        CellValue::Array(row.iter().map(|&val| CellValue::Float(val)).collect())
                    })
                    .chain(std::iter::once(CellValue::Array(
                        op.translation
                            .iter()
                            .map(|&val| CellValue::Float(val))
                            .collect(),
                    )))
            })
            .collect::<Vec<CellValue>>();

        Cell::Block("SYMMETRY_OPS", block_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_symmetry_ops_serde() {
        // Define a test struct matching the expected .cell file structure
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileSymmetryOps {
            symmetry_ops: SymmetryOps,
        }

        // Test string based on the documentation example (showing 1 full operation)
        let symmetry_ops_str = r#"%BLOCK SYMMETRY_OPS
   -1.0000000000    0.0000000000    0.0000000000
    0.0000000000   -1.0000000000    0.0000000000
    0.0000000000    0.0000000000    1.0000000000
    0.5000000000    0.0000000000    0.5000000000
    1.000000000000000       0.000000000000000       0.000000000000000
    0.000000000000000       1.000000000000000       0.000000000000000
    0.000000000000000       0.000000000000000       1.000000000000000
    0.000000000000000        0.000000000000000        0.000000000000000
    -1.000000000000000       0.000000000000000       0.000000000000000
    0.000000000000000      -1.000000000000000       0.000000000000000
    0.000000000000000       0.000000000000000      -1.000000000000000
    0.000000000000000        0.000000000000000        0.000000000000000
%ENDBLOCK SYMMETRY_OPS
"#;

        // 1. Test Deserialization
        let cell_file_result: Result<CellFileSymmetryOps, _> = from_str(symmetry_ops_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();

        // Debug print the deserialized struct
        println!("Deserialized SYMMETRY_OPS: {:#?}", cell_file.symmetry_ops);

        // Verify the content
        assert_eq!(cell_file.symmetry_ops.ops.len(), 3);
        let op = &cell_file.symmetry_ops.ops[0];
        assert_eq!(op.rotation[0], [-1.0, 0.0, 0.0]);
        assert_eq!(op.rotation[1], [0.0, -1.0, 0.0]);
        assert_eq!(op.rotation[2], [0.0, 0.0, 1.0]);
        assert_eq!(op.translation, [0.5, 0.0, 0.5]);

        // 2. Test Serialization using ToCell
        let serialized_result = to_string(&cell_file.symmetry_ops.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        // Print the serialized string
        println!("Serialized SYMMETRY_OPS:\n{serialized_string}"); // Clippy: use {} instead of {:?} for String

        // Basic checks on the serialized string
        assert!(serialized_string.contains("%BLOCK SYMMETRY_OPS"));
        assert!(serialized_string.contains("%ENDBLOCK SYMMETRY_OPS"));
        assert!(serialized_string.contains("-1.0"));
        assert!(serialized_string.contains("0.5"));
        // Note: Exact string comparison can be brittle due to formatting/whitespace/order
    }
}
