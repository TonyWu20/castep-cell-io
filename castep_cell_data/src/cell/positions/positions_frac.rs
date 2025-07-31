#![allow(dead_code)]
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

use crate::cell::species::Species;

/// Represents a single atom entry within the POSITIONS_FRAC block.
///
/// Consists of the element symbol/number, fractional coordinates, and an optional spin.
/// Format: <element> <x> <y> <z> [SPIN <value>]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "PositionFracEntryRepr")] // Use intermediate repr for deserialization
pub struct PositionFracEntry {
    /// The chemical element symbol (e.g., "Fe") or atomic number as a string (e.g., "26").
    pub species: Species,
    /// Fractional coordinates [x, y, z].
    pub coord: [f64; 3],
    /// Optional initial spin polarization for the atom.
    pub spin: Option<f64>,
}

/// Intermediate representation for deserializing a `PositionFracEntry`.
///
/// Handles the two possible line formats:
/// 1. Element X Y Z
/// 2. Element X Y Z SPIN Value
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum PositionFracEntryRepr {
    /// Format: Element X Y Z
    Essential(Species, f64, f64, f64),
    /// Format: Element X Y Z SPIN Value
    /// Note: We ignore the literal "SPIN" string.
    WithSpin(Species, f64, f64, f64, String, f64), // String for "SPIN" or "SPIN="
}

impl From<PositionFracEntryRepr> for PositionFracEntry {
    /// Converts the intermediate representation into the final struct.
    fn from(repr: PositionFracEntryRepr) -> Self {
        match repr {
            PositionFracEntryRepr::Essential(element, x, y, z) => Self {
                species: element,
                coord: [x, y, z],
                spin: None,
            },
            PositionFracEntryRepr::WithSpin(element, x, y, z, _spin_keyword, spin_value) => {
                // _spin_keyword is "SPIN" or "SPIN=", which we don't need to store
                Self {
                    species: element,
                    coord: [x, y, z],
                    spin: Some(spin_value),
                }
            }
        }
    }
}

impl ToCellValue for PositionFracEntry {
    /// Converts the entry into a `CellValue::Array` representing one line of the block.
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(
            [self.species.to_cell_value()]
                .into_iter()
                .chain(self.coord.into_iter().map(CellValue::Float))
                .chain(
                    self.spin
                        .as_ref()
                        .map(|spin| {
                            [
                                CellValue::String("SPIN=".to_string()),
                                CellValue::Float(*spin),
                            ]
                            .to_vec()
                        })
                        .unwrap_or([CellValue::Null].to_vec()),
                )
                .collect(),
        )
    }
}

/// Represents the POSITIONS_FRAC block.
///
/// Contains a list of atomic positions in fractional coordinates.
/// Format:
/// %BLOCK POSITIONS_FRAC
/// Species1/I1 R1i R1j R1k [SPIN S1]
/// Species2/I2 R2i R2j R2k [SPIN S2]
/// ...
/// %ENDBLOCK POSITIONS_FRAC
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "POSITIONS_FRAC")] // Ensure correct block name during serde
#[serde(transparent)] // Serialize/Deserialize as if it's directly the Vec
pub struct PositionsFrac {
    /// The list of atom entries.
    pub positions: Vec<PositionFracEntry>,
}

impl ToCell for PositionsFrac {
    /// Converts the block into the intermediate `Cell` representation for serialization.
    fn to_cell(&self) -> Cell {
        Cell::Block(
            "POSITIONS_FRAC", // Block name
            self.positions
                .iter()
                .map(|entry| entry.to_cell_value()) // Convert each entry to CellValue::Array
                .collect::<Vec<CellValue>>(), // Collect into Vec for the block content
        )
    }
}

// You might also want a top-level struct to represent a .cell file containing this block
// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// pub struct CellFileWithPositions {
//     pub positions_frac: PositionsFrac,
// }

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_positions_frac_serde() {
        // Define a test struct matching the expected .cell file structure
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFilePositions {
            positions_frac: PositionsFrac,
        }

        // Test string with different SPIN formats
        let positions_frac_str = r#"%BLOCK POSITIONS_FRAC
Fe 0.00    0.00    0.00
Fe 0.50    0.50    0.00 SPIN= 4
Fe 0.00    0.50    0.50 SPIN 4
26 0.75 0.25 0.75
%ENDBLOCK POSITIONS_FRAC
"#;

        // 1. Test Deserialization
        let cell_file_result: Result<CellFilePositions, _> = from_str(positions_frac_str);
        // Assert successful deserialization
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();

        // Debug print the deserialized struct
        println!(
            "Deserialized POSITIONS_FRAC: {:#?}",
            cell_file.positions_frac
        );

        // Verify the content
        assert_eq!(cell_file.positions_frac.positions.len(), 4);
        assert_eq!(
            cell_file.positions_frac.positions[0].species,
            Species::Symbol("Fe".to_string())
        );
        assert_eq!(
            cell_file.positions_frac.positions[0].coord,
            [0.00, 0.00, 0.00]
        );
        assert_eq!(cell_file.positions_frac.positions[0].spin, None);

        assert_eq!(
            cell_file.positions_frac.positions[1].species,
            Species::Symbol("Fe".to_string())
        );
        assert_eq!(
            cell_file.positions_frac.positions[1].coord,
            [0.50, 0.50, 0.00]
        );
        assert_eq!(cell_file.positions_frac.positions[1].spin, Some(4.0)); // SPIN= 4

        assert_eq!(
            cell_file.positions_frac.positions[2].species,
            Species::Symbol("Fe".to_string())
        );
        assert_eq!(
            cell_file.positions_frac.positions[2].coord,
            [0.00, 0.50, 0.50]
        );
        assert_eq!(cell_file.positions_frac.positions[2].spin, Some(4.0)); // SPIN 4

        assert_eq!(
            cell_file.positions_frac.positions[3].species,
            Species::AtomicNumber(26)
        );
        assert_eq!(
            cell_file.positions_frac.positions[3].coord,
            [0.75, 0.25, 0.75]
        );
        assert_eq!(cell_file.positions_frac.positions[3].spin, None);

        // 2. Test Serialization using ToCell
        let serialized_result = to_string(&cell_file.positions_frac.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        // Print the serialized string
        println!("Serialized POSITIONS_FRAC:\n{serialized_string}");

        // Basic checks on the serialized string
        assert!(serialized_string.contains("%BLOCK POSITIONS_FRAC"));
        assert!(serialized_string.contains("%ENDBLOCK POSITIONS_FRAC"));
        assert!(serialized_string.contains("Fe"));
        assert!(serialized_string.contains("26"));
        assert!(serialized_string.contains("SPIN")); // Check if SPIN parts are serialized
        // Note: Exact string comparison can be brittle due to formatting/whitespace/order
    }
}
