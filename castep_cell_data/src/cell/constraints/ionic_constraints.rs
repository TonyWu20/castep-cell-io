use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

use crate::cell::species::Species;

/// Represents a single constraint entry within the IONIC_CONSTRAINTS block.
///
/// # Format:
///   Ic CCC/Ic In Rix Riy Riz
/// Where:
/// - Ic: Constraint number
/// - CCC/Ic: Species (symbol or atomic number)
/// - In: Ion number within the species
/// - Rix, Riy, Riz: Coefficients for x, y, z Cartesian coordinates
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "ICRepr")]
pub struct IonicConstraintEntry {
    /// The constraint number.
    pub constraint_number: u32,
    /// The species of the ion.
    pub species: Species,
    /// The ion number within the species (1-based index).
    pub ion_number: u32,
    /// Coefficients for the Cartesian coordinates [x, y, z].
    pub coefficients: [f64; 3],
}

#[derive(Debug, Deserialize)]
struct ICRepr {
    /// The constraint number.
    pub constraint_number: u32,
    /// The species of the ion.
    pub species: Species,
    /// The ion number within the species (1-based index).
    pub ion_number: u32,
    /// Coefficients for the Cartesian coordinates [x, y, z].
    pub ri_x: f64,
    /// Coefficients for the Cartesian coordinates [x, y, z].
    pub ri_y: f64,
    /// Coefficients for the Cartesian coordinates [x, y, z].
    pub ri_z: f64,
}

impl From<ICRepr> for IonicConstraintEntry {
    fn from(value: ICRepr) -> Self {
        let ICRepr {
            constraint_number,
            species,
            ion_number,
            ri_x,
            ri_y,
            ri_z,
        } = value;
        Self {
            constraint_number,
            species,
            ion_number,
            coefficients: [ri_x, ri_y, ri_z],
        }
    }
}

// Assuming the line format is straightforward and matches the field order,
// standard derives should work with castep_cell_serde.
// If needed, an intermediate repr like in positions_frac can be added.

impl ToCellValue for IonicConstraintEntry {
    /// Converts the entry into a `CellValue::Array` representing one line of the block.
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(
            vec![
                CellValue::UInt(self.constraint_number),
                self.species.to_cell_value(),
                CellValue::UInt(self.ion_number),
            ]
            .into_iter()
            .chain(self.coefficients.into_iter().map(CellValue::Float))
            .collect(),
        )
    }
}

/// Represents the IONIC_CONSTRAINTS block.
///
/// Defines linear constraints on changes to Cartesian ionic positions.
/// Format:
/// %BLOCK IONIC_CONSTRAINTS
/// I1 CCC1/I1 In1 R1i R1j R1k
/// I2 CCC2/I2 In2 R2i R2j R2k
/// ...
/// %ENDBLOCK IONIC_CONSTRAINTS
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "IONIC_CONSTRAINTS")] // Ensure correct block name during serde
#[serde(transparent)] // Serialize/Deserialize as if it's directly the Vec
pub struct IonicConstraints {
    /// The list of constraint entries.
    pub constraints: Vec<IonicConstraintEntry>,
}

impl ToCell for IonicConstraints {
    /// Converts the block into the intermediate `Cell` representation for serialization.
    fn to_cell(&self) -> Cell {
        Cell::Block(
            "IONIC_CONSTRAINTS", // Block name
            self.constraints
                .iter()
                .map(|entry| entry.to_cell_value()) // Convert each entry to CellValue::Array
                .collect::<Vec<CellValue>>(), // Collect into Vec for the block content
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_ionic_constraints_serde() {
        // Define a test struct matching the expected .cell file structure
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileConstraints {
            ionic_constraints: IonicConstraints,
        }

        // Test string based on the documentation example
        let ionic_constraints_str = r#"%BLOCK IONIC_CONSTRAINTS
       1       W       1    1.0000000000    0.0000000000    0.0000000000
       2       W       1    0.0000000000    1.0000000000    0.0000000000
       3       W       1    0.0000000000    0.0000000000    1.0000000000
       4       W       2    1.0000000000    0.0000000000    0.0000000000
       5       W       2    0.0000000000    1.0000000000    0.0000000000
       6       W       2    0.0000000000    0.0000000000    1.0000000000
       7       W       3    1.0000000000    0.0000000000    0.0000000000
       8       W       3    0.0000000000    1.0000000000    0.0000000000
       9       W       3    0.0000000000    0.0000000000    1.0000000000
      10       W       4    1.0000000000    0.0000000000    0.0000000000
      11       W       4    0.0000000000    1.0000000000    0.0000000000
      12       W       4    0.0000000000    0.0000000000    1.0000000000
%ENDBLOCK IONIC_CONSTRAINTS
"#;

        // 1. Test Deserialization
        let cell_file_result: Result<CellFileConstraints, _> = from_str(ionic_constraints_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();

        // Debug print the deserialized struct
        println!(
            "Deserialized IONIC_CONSTRAINTS: {:#?}",
            cell_file.ionic_constraints
        );

        // Verify the content
        assert_eq!(cell_file.ionic_constraints.constraints.len(), 12);
        // Check first entry
        assert_eq!(
            cell_file.ionic_constraints.constraints[0].constraint_number,
            1
        );
        assert_eq!(
            cell_file.ionic_constraints.constraints[0].species,
            Species::Symbol("W".to_string())
        );
        assert_eq!(cell_file.ionic_constraints.constraints[0].ion_number, 1);
        assert_eq!(
            cell_file.ionic_constraints.constraints[0].coefficients,
            [1.0, 0.0, 0.0]
        );
        // Check last entry
        assert_eq!(
            cell_file.ionic_constraints.constraints[11].constraint_number,
            12
        );
        assert_eq!(
            cell_file.ionic_constraints.constraints[11].species,
            Species::Symbol("W".to_string())
        );
        assert_eq!(cell_file.ionic_constraints.constraints[11].ion_number, 4);
        assert_eq!(
            cell_file.ionic_constraints.constraints[11].coefficients,
            [0.0, 0.0, 1.0]
        );

        // Test with atomic number
        let ionic_constraints_with_atomic_number_str = r#"%BLOCK IONIC_CONSTRAINTS
1 74 1 1.0 0.0 0.0
%ENDBLOCK IONIC_CONSTRAINTS
"#;
        let cell_file_result2: Result<CellFileConstraints, _> =
            from_str(ionic_constraints_with_atomic_number_str);
        assert!(
            cell_file_result2.is_ok(),
            "Deserialization with atomic number failed: {:?}",
            cell_file_result2.err()
        );
        let cell_file2 = cell_file_result2.unwrap();
        assert_eq!(
            cell_file2.ionic_constraints.constraints[0].species,
            Species::AtomicNumber(74)
        );

        // 2. Test Serialization using ToCell
        let serialized_result = to_string(&cell_file.ionic_constraints.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        // Print the serialized string
        println!("Serialized IONIC_CONSTRAINTS:\n{serialized_string}");

        // Basic checks on the serialized string
        assert!(serialized_string.contains("%BLOCK IONIC_CONSTRAINTS"));
        assert!(serialized_string.contains("%ENDBLOCK IONIC_CONSTRAINTS"));
        assert!(serialized_string.contains("W"));
        assert!(serialized_string.contains("12")); // Last constraint number
        // Note: Exact string comparison can be brittle due to formatting/whitespace/order
        let serialized_result2 = to_string(&cell_file2.ionic_constraints.to_cell());
        assert!(
            serialized_result2.is_ok(),
            "Serialization failed: {:?}",
            serialized_result2.err()
        );
        let serialized_string2 = serialized_result2.unwrap();
        // Print the serialized string
        println!("Serialized IONIC_CONSTRAINTS:\n{serialized_string2}");
        assert!(serialized_string2.contains("74")); // From the second test
        // Test string based on the documentation example
        let ionic_constraints_str_empty = r#"%BLOCK IONIC_CONSTRAINTS
%ENDBLOCK IONIC_CONSTRAINTS
"#;
        let cell_file_result3: Result<CellFileConstraints, _> =
            from_str(ionic_constraints_str_empty);
        assert!(
            cell_file_result3.is_ok(),
            "Deserialization with atomic number failed: {:?}",
            cell_file_result3.err()
        );
        let cell_file3 = cell_file_result3.unwrap();
        let serialized_result3 = to_string(&cell_file3.ionic_constraints.to_cell());
        assert!(
            serialized_result3.is_ok(),
            "Serialization failed: {:?}",
            serialized_result3.err()
        );
        let serialized_string3 = serialized_result3.unwrap();
        println!("Serialized empty IONIC_CONSTRAINTS:\n{serialized_string3}");
    }
}
