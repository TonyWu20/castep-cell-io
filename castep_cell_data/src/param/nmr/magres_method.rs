use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Selects the method used by CASTEP for the evaluation of the quantum-mechanical position operator.
///
/// Keyword type: String
///
/// Default: MagresMethod::Crystal
///
/// Example:
/// MAGRES_METHOD : molecular
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MAGRES_METHOD")]
pub enum MagresMethod {
    /// Uses the reciprocal space representation; applicable for both crystals and "molecule in a box"
    #[serde(alias = "crystal", alias = "CRYSTAL")]
    Crystal,
    /// Applicable only for "molecule in a box"; faster calculation
    #[serde(alias = "molecular", alias = "MOLECULAR")]
    Molecular,
}

impl Default for MagresMethod {
    fn default() -> Self {
        Self::Crystal // Default is Crystal
    }
}

impl ToCell for MagresMethod {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MAGRES_METHOD", self.to_cell_value())
    }
}

impl ToCellValue for MagresMethod {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                MagresMethod::Crystal => "Crystal",
                MagresMethod::Molecular => "Molecular",
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
    fn test_magres_method_serde() {
        // Test Deserialization for various cases
        let test_cases_deser = [
            ("MAGRES_METHOD : Crystal", MagresMethod::Crystal),
            ("MAGRES_METHOD : crystal", MagresMethod::Crystal),
            ("MAGRES_METHOD : CRYSTAL", MagresMethod::Crystal), // Uppercase alias
            ("MAGRES_METHOD : Molecular", MagresMethod::Molecular),
            ("MAGRES_METHOD : molecular", MagresMethod::Molecular),
            ("MAGRES_METHOD : MOLECULAR", MagresMethod::Molecular), // Uppercase alias
        ];

        for (input_str, expected_method) in test_cases_deser {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithMagresMethod {
                magres_method: MagresMethod,
            }

            let cell_file_result: Result<CellFileWithMagresMethod, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.magres_method, expected_method,
                "Failed for input: {input_str}"
            );
        }

        // Test Serialization
        let magres_method_instance = MagresMethod::Molecular;
        let serialized_result = to_string(&magres_method_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MAGRES_METHOD (Molecular): {serialized_string}");
        assert!(serialized_string.contains("MAGRES_METHOD"));
        assert!(serialized_string.contains("Molecular"));

        // Test Default
        assert_eq!(MagresMethod::default(), MagresMethod::Crystal);
    }
}
