use crate::units::LengthUnit;
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the tolerance within which symmetry will be considered to be satisfied.
/// If an ion is found within this distance of its symmetric position, the symmetry
/// will be considered to be satisfied. Unit of length must be specified.
///
/// Keyword type: Real
///
/// Default: 0.01 ang
///
/// Example:
/// SYMMETRY_TOL : 0.25 ang
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "SYMMETRY_TOL")] // Ensures correct key name during serde
pub struct SymmetryTol {
    /// The tolerance value.
    pub value: f64,
    /// The unit of length for the tolerance.
    pub unit: LengthUnit,
}

// Implement ToCell for SymmetryTol to enable serialization via your custom backend
impl ToCell for SymmetryTol {
    fn to_cell(&self) -> Cell {
        // Create a KeyValue Cell with the name "SYMMETRY_TOL" and an Array value
        // containing the float and the unit string.
        Cell::KeyValue(
            "SYMMETRY_TOL",
            CellValue::Array(vec![
                CellValue::Float(self.value),
                self.unit.to_cell_value(), // Converts LengthUnit to CellValue::String
            ]),
        )
    }
}

// Implement ToCellValue for SymmetryTol.
// This allows it to be used as a value within other structures if needed.
impl ToCellValue for SymmetryTol {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(vec![
            CellValue::Float(self.value),
            self.unit.to_cell_value(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_symmetry_tol_serde() {
        // 1. Test Deserialization
        // Note: This test assumes the deserializer can correctly parse
        // "SYMMETRY_TOL : 0.25 ang" into a structure with value and unit.
        // If direct derive doesn't work, the SymmetryTolRepr + From approach is needed.
        let symmetry_tol_str = "SYMMETRY_TOL : 0.25 ang";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithSymmetryTol {
            symmetry_tol: SymmetryTol,
        }

        let cell_file_result: Result<CellFileWithSymmetryTol, _> = from_str(symmetry_tol_str);
        // Assert successful deserialization (assuming the parser handles it)
        // If this fails, implement the custom Deserialize as shown above.
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        dbg!(&cell_file);
        assert!((cell_file.symmetry_tol.value - 0.25).abs() < f64::EPSILON);
        assert_eq!(cell_file.symmetry_tol.unit, LengthUnit::Ang);

        // 2. Test Serialization using ToCell
        let symmetry_tol_instance = SymmetryTol {
            value: 0.01,
            unit: LengthUnit::Bohr,
        };
        let serialized_result = to_string(&symmetry_tol_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        println!("Serialized SYMMETRY_TOL (0.01 bohr): {serialized_string}"); // Clippy suggestion
        // Basic checks
        assert!(serialized_string.contains("SYMMETRY_TOL"));
        assert!(serialized_string.contains("0.01"));
        assert!(serialized_string.contains("bohr"));
    }
}
