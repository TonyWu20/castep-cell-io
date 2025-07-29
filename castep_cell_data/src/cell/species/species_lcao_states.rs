use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

use super::Species;

/// Represents a single entry within the SPECIES_LCAO_STATES block,
/// linking a species to the number of LCAO states for it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeciesLcaoState {
    /// The species (symbol or atomic number).
    pub species: Species,
    /// The number of angular momentum channels (LCAO states) for this species.
    pub num_states: u32,
}

// Implement ToCellValue for SpeciesLcaoState to enable serialization of individual lines.
impl ToCellValue for SpeciesLcaoState {
    /// Converts the entry into a `CellValue::Array` representing one line of the block.
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(vec![
            self.species.to_cell_value(), // Converts Species to CellValue::String or CellValue::UInt
            CellValue::UInt(self.num_states),
        ])
    }
}

/// Represents the SPECIES_LCAO_STATES block.
///
/// Defines the size of the LCAO basis set used for population analysis.
/// Format:
/// %BLOCK SPECIES_LCAO_STATES
/// CCC1/I1 IB1
/// CCC2/I2 IB2
/// ...
/// %ENDBLOCK SPECIES_LCAO_STATES
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "SPECIES_LCAO_STATES")] // Ensure correct block name during serde
#[serde(transparent)] // Serialize/Deserialize as if it's directly the Vec
pub struct SpeciesLcaoStates {
    /// The list of species and their corresponding LCAO state counts.
    pub states: Vec<SpeciesLcaoState>,
}

impl ToCell for SpeciesLcaoStates {
    /// Converts the block into the intermediate `Cell` representation for serialization.
    fn to_cell(&self) -> Cell {
        Cell::Block(
            "SPECIES_LCAO_STATES", // Block name
            self.states
                .iter()
                .map(|state_entry| state_entry.to_cell_value()) // Convert each entry to CellValue::Array
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
    fn test_species_lcao_states_serde() {
        // Define a test struct matching the expected .cell file structure
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileLcaoStates {
            species_lcao_states: SpeciesLcaoStates,
        }

        // Test string based on the documentation example
        let species_lcao_states_str = r#"%BLOCK SPECIES_LCAO_STATES
       O         2
      Al         2
      Ti         3
      Cs         4
%ENDBLOCK SPECIES_LCAO_STATES
"#;

        // 1. Test Deserialization
        let cell_file_result: Result<CellFileLcaoStates, _> = from_str(species_lcao_states_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();

        // Debug print the deserialized struct
        println!(
            "Deserialized SPECIES_LCAO_STATES: {:#?}",
            cell_file.species_lcao_states
        );

        // Verify the content
        assert_eq!(cell_file.species_lcao_states.states.len(), 4);
        // Check first entry
        assert_eq!(
            cell_file.species_lcao_states.states[0].species,
            Species::Symbol("O".to_string())
        );
        assert_eq!(cell_file.species_lcao_states.states[0].num_states, 2);
        // Check last entry
        assert_eq!(
            cell_file.species_lcao_states.states[3].species,
            Species::Symbol("Cs".to_string())
        );
        assert_eq!(cell_file.species_lcao_states.states[3].num_states, 4);

        // 2. Test Serialization using ToCell
        let test_states = SpeciesLcaoStates {
            states: vec![
                SpeciesLcaoState {
                    species: Species::Symbol("Fe".to_string()),
                    num_states: 3,
                },
                SpeciesLcaoState {
                    species: Species::AtomicNumber(8), // Oxygen
                    num_states: 2,
                },
            ],
        };

        let serialized_result = to_string(&test_states.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        // Print the serialized string
        println!("Serialized SPECIES_LCAO_STATES:\n{serialized_string}"); // Clippy suggestion

        // Basic checks on the serialized string
        assert!(serialized_string.contains("%BLOCK SPECIES_LCAO_STATES"));
        assert!(serialized_string.contains("%ENDBLOCK SPECIES_LCAO_STATES"));
        assert!(serialized_string.contains("Fe"));
        assert!(serialized_string.contains("8")); // Atomic number for Oxygen
        assert!(serialized_string.contains("3")); // States for Fe
        assert!(serialized_string.contains("2")); // States for O
        // Note: Exact string comparison can be brittle due to formatting/whitespace/order
    }
}
