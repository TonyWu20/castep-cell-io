use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies how many of the lowest lying modes to ignore for ionic permittivity/polarizability.
///
/// Keyword type: String
///
/// Default: EfieldIgnoreMolModes::Crystal
///
/// Example:
/// EFIELD_IGNORE_MOL_MODES : Molecule
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "EFIELD_IGNORE_MOL_MODES")]
pub enum EfieldIgnoreMolModes {
    /// Ignore the three lowest lying modes
    #[serde(alias = "crystal", alias = "CRYSTAL")]
    Crystal,
    /// Ignore the six lowest lying modes
    #[serde(alias = "molecule", alias = "MOLECULE")]
    Molecule,
    /// Ignore the five lowest lying modes
    #[serde(
        alias = "linear_molecule",
        alias = "LINEAR_MOLECULE",
        alias = "Linear_molecule"
    )]
    LinearMolecule,
}

impl Default for EfieldIgnoreMolModes {
    fn default() -> Self {
        Self::Crystal // Default is Crystal
    }
}

impl ToCell for EfieldIgnoreMolModes {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("EFIELD_IGNORE_MOL_MODES", self.to_cell_value())
    }
}

impl ToCellValue for EfieldIgnoreMolModes {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                EfieldIgnoreMolModes::Crystal => "Crystal",
                EfieldIgnoreMolModes::Molecule => "Molecule",
                EfieldIgnoreMolModes::LinearMolecule => "Linear_molecule",
            }
            .to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_efield_ignore_mol_modes_serde() {
        // Test Deserialization for various cases
        let test_cases_deser = [
            (
                "EFIELD_IGNORE_MOL_MODES : Crystal",
                EfieldIgnoreMolModes::Crystal,
            ),
            (
                "EFIELD_IGNORE_MOL_MODES : crystal",
                EfieldIgnoreMolModes::Crystal,
            ),
            (
                "EFIELD_IGNORE_MOL_MODES : CRYSTAL",
                EfieldIgnoreMolModes::Crystal,
            ), // Uppercase alias
            (
                "EFIELD_IGNORE_MOL_MODES : Molecule",
                EfieldIgnoreMolModes::Molecule,
            ),
            (
                "EFIELD_IGNORE_MOL_MODES : molecule",
                EfieldIgnoreMolModes::Molecule,
            ),
            (
                "EFIELD_IGNORE_MOL_MODES : MOLECULE",
                EfieldIgnoreMolModes::Molecule,
            ), // Uppercase alias
            (
                "EFIELD_IGNORE_MOL_MODES : Linear_molecule",
                EfieldIgnoreMolModes::LinearMolecule,
            ),
            (
                "EFIELD_IGNORE_MOL_MODES : linear_molecule",
                EfieldIgnoreMolModes::LinearMolecule,
            ),
            (
                "EFIELD_IGNORE_MOL_MODES : LINEAR_MOLECULE",
                EfieldIgnoreMolModes::LinearMolecule,
            ), // Uppercase alias
        ];

        for (input_str, expected_mode) in test_cases_deser {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithEfieldIgnoreMolModes {
                efield_ignore_mol_modes: EfieldIgnoreMolModes,
            }

            let cell_file_result: Result<CellFileWithEfieldIgnoreMolModes, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.efield_ignore_mol_modes, expected_mode,
                "Failed for input: {input_str}"
            );
        }

        // Test Serialization
        let efield_ignore_mol_modes_instance = EfieldIgnoreMolModes::Molecule;
        let serialized_result = to_string(&efield_ignore_mol_modes_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized EFIELD_IGNORE_MOL_MODES (Molecule): {serialized_string}");
        assert!(serialized_string.contains("EFIELD_IGNORE_MOL_MODES"));
        assert!(serialized_string.contains("Molecule"));

        // Test Default
        assert_eq!(
            EfieldIgnoreMolModes::default(),
            EfieldIgnoreMolModes::Crystal
        );
    }
}
