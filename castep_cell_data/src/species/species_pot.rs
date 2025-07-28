// File: species_pot.rs (or part of your main module structure)

use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
// Assuming Species is defined in a common module or the parent module
use super::Species; // As specified in your instruction
use serde::{Deserialize, Serialize};

/// Represents a single entry within the SPECIES_POT block,
/// linking a species to its pseudopotential filename.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeciesPotEntry {
    /// The species (symbol or atomic number).
    pub species: Species,
    /// The filename of the pseudopotential.
    pub filename: String,
}

// Implement ToCellValue for SpeciesPotEntry to enable serialization of individual lines.
impl ToCellValue for SpeciesPotEntry {
    /// Converts the entry into a `CellValue::Array` representing one line of the block.
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(vec![
            self.species.to_cell_value(), // Converts Species to CellValue::String or CellValue::UInt
            CellValue::String(self.filename.clone()), // Use CellValue::String for the filename
        ])
    }
}

/// Represents the SPECIES_POT block.
///
/// Defines the pseudopotential files associated with each atomic species.
/// Format:
/// %BLOCK SPECIES_POT
/// CCC1/I1 [filename]
/// CCC2/I2 [filename]
/// ...
/// %ENDBLOCK SPECIES_POT
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "SPECIES_POT")] // Ensure correct block name during serde
#[serde(transparent)] // Serialize/Deserialize as if it's directly the Vec of entries
pub struct SpeciesPot {
    /// The list of species and their corresponding pseudopotential filenames.
    pub potentials: Vec<SpeciesPotEntry>,
}

impl ToCell for SpeciesPot {
    /// Converts the block into the intermediate `Cell` representation for serialization.
    fn to_cell(&self) -> Cell {
        Cell::Block(
            "SPECIES_POT", // Block name
            self.potentials
                .iter()
                .map(|pot_entry| pot_entry.to_cell_value()) // Convert each entry to CellValue::Array
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
    fn test_species_pot_serde() {
        // Define a test struct matching the expected .cell file structure
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileSpeciesPot {
            species_pot: SpeciesPot,
        }

        // Test string based on the documentation example
        let species_pot_str = r#"%BLOCK SPECIES_POT
       O  O_00.usp
      Al  Al_00.usp
      Ti  Ti_00.uspcc
      Cs  Cs_00.usp
%ENDBLOCK SPECIES_POT
"#;

        // 1. Test Deserialization
        let cell_file_result: Result<CellFileSpeciesPot, _> = from_str(species_pot_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();

        // Debug print the deserialized struct
        println!("Deserialized SPECIES_POT: {:#?}", cell_file.species_pot);

        // Verify the content
        assert_eq!(cell_file.species_pot.potentials.len(), 4);
        // Check first entry
        assert_eq!(
            cell_file.species_pot.potentials[0].species,
            Species::Symbol("O".to_string())
        );
        assert_eq!(cell_file.species_pot.potentials[0].filename, "O_00.usp");
        // Check last entry
        assert_eq!(
            cell_file.species_pot.potentials[3].species,
            Species::Symbol("Cs".to_string())
        );
        assert_eq!(cell_file.species_pot.potentials[3].filename, "Cs_00.usp");

        // 2. Test Serialization using ToCell
        let test_pots = SpeciesPot {
            potentials: vec![
                SpeciesPotEntry {
                    species: Species::Symbol("Fe".to_string()),
                    filename: "Fe_01.usp".to_string(),
                },
                SpeciesPotEntry {
                    species: Species::AtomicNumber(8), // Oxygen
                    filename: "O_00.usp".to_string(),
                },
            ],
        };

        let serialized_result = to_string(&test_pots.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        // Print the serialized string
        println!("Serialized SPECIES_POT:\n{serialized_string}"); // Clippy suggestion

        // Basic checks on the serialized string
        assert!(serialized_string.contains("%BLOCK SPECIES_POT"));
        assert!(serialized_string.contains("%ENDBLOCK SPECIES_POT"));
        assert!(serialized_string.contains("Fe"));
        assert!(serialized_string.contains("8")); // Atomic number for Oxygen
        assert!(serialized_string.contains("Fe_01.usp"));
        assert!(serialized_string.contains("O_00.usp"));
        // Note: Exact string comparison can be brittle due to formatting/whitespace/order
    }
}
