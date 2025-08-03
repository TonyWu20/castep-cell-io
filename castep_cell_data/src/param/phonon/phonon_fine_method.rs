use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Selects which calculation method to use for phonon calculation on a fine grid.
///
/// Keyword type: String
///
/// Default: PhononFineMethod::None
///
/// Example:
/// PHONON_FINE_METHOD : SUPERCELL
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "PHONON_FINE_METHOD")]
pub enum PhononFineMethod {
    /// No interpolation, direct calculations
    #[serde(alias = "none", alias = "NONE")]
    None,
    /// Use Linear response (density functional perturbation theory) method
    #[serde(alias = "interpolate", alias = "INTERPOLATE")]
    Interpolate,
    /// Use Finite displacement method
    #[serde(alias = "supercell", alias = "SUPERCELL")]
    Supercell,
}

impl Default for PhononFineMethod {
    fn default() -> Self {
        Self::None // Default is NONE
    }
}

impl ToCell for PhononFineMethod {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PHONON_FINE_METHOD", self.to_cell_value())
    }
}

impl ToCellValue for PhononFineMethod {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                PhononFineMethod::None => "NONE",
                PhononFineMethod::Interpolate => "INTERPOLATE",
                PhononFineMethod::Supercell => "SUPERCELL",
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
    fn test_phonon_fine_method_serde() {
        // Test Deserialization for various cases
        let test_cases_deser = [
            ("PHONON_FINE_METHOD : NONE", PhononFineMethod::None),
            ("PHONON_FINE_METHOD : none", PhononFineMethod::None),
            ("PHONON_FINE_METHOD : NONE", PhononFineMethod::None), // Uppercase alias
            (
                "PHONON_FINE_METHOD : INTERPOLATE",
                PhononFineMethod::Interpolate,
            ),
            (
                "PHONON_FINE_METHOD : interpolate",
                PhononFineMethod::Interpolate,
            ),
            (
                "PHONON_FINE_METHOD : INTERPOLATE",
                PhononFineMethod::Interpolate,
            ), // Uppercase alias
            (
                "PHONON_FINE_METHOD : SUPERCELL",
                PhononFineMethod::Supercell,
            ),
            (
                "PHONON_FINE_METHOD : supercell",
                PhononFineMethod::Supercell,
            ),
            (
                "PHONON_FINE_METHOD : SUPERCELL",
                PhononFineMethod::Supercell,
            ), // Uppercase alias
        ];

        for (input_str, expected_method) in test_cases_deser {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithPhononFineMethod {
                phonon_fine_method: PhononFineMethod,
            }

            let cell_file_result: Result<CellFileWithPhononFineMethod, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.phonon_fine_method, expected_method,
                "Failed for input: {input_str}"
            );
        }

        // Test Serialization
        let phonon_fine_method_instance = PhononFineMethod::Supercell;
        let serialized_result = to_string(&phonon_fine_method_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized PHONON_FINE_METHOD (SUPERCELL): {serialized_string}");
        assert!(serialized_string.contains("PHONON_FINE_METHOD"));
        assert!(serialized_string.contains("SUPERCELL"));

        // Test Default
        assert_eq!(PhononFineMethod::default(), PhononFineMethod::None);
    }
}
