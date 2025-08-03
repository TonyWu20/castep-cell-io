// Note: This file handles both SECONDD_METHOD and the obsolete PHONON_METHOD.
// We'll define the enum and logic for SECONDD_METHOD primarily.
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Alias to `enum SeconddMethod`
pub type PhononMethod = SeconddMethod;

/// Specifies which calculation method will be used for phonons (SECONDD_METHOD).
/// Obsolete PHONON_METHOD is handled by aliasing the keyword name.
/// You might specify #[serde(alias = "PHONON_METHOD", alias="phonon_method")]
/// for backward compatibility
///
/// Keyword type: String
///
/// Default: SecondMethod::LinearResponse
///
/// Example:
/// SECONDD_METHOD : FINITEDISPLACEMENT
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "SECONDD_METHOD")]
// Handle the obsolete PHONON_METHOD name
// Note: Serde typically handles one primary name. Handling aliases for the *keyword itself*
// (not enum variants) usually requires custom deserializer logic or multiple struct fields.
// For simplicity here, we define it for SECONDD_METHOD. A wrapper struct or custom logic
// in the parent deserializer might be needed to fully support PHONON_METHOD alias.
pub enum SeconddMethod {
    /// Linear response method (or density functional perturbation theory)
    #[serde(alias = "linearresponse", alias = "LINEARRESPONSE")]
    #[serde(alias = "DFPT", alias = "dfpt")] // Alternative name
    LinearResponse,
    /// Finite displacement method
    #[serde(alias = "finitedisplacement", alias = "FINITEDISPLACEMENT")]
    FiniteDisplacement,
}

impl Default for SeconddMethod {
    fn default() -> Self {
        Self::LinearResponse // Default is LINEARRESPONSE
    }
}

impl ToCell for SeconddMethod {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("SECONDD_METHOD", self.to_cell_value())
    }
}

impl ToCellValue for SeconddMethod {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                SeconddMethod::LinearResponse => "LINEARRESPONSE", // Or "DFPT"
                SeconddMethod::FiniteDisplacement => "FINITEDISPLACEMENT",
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
    fn test_second_method_serde() {
        // Test Deserialization for various cases (SECONDD_METHOD)
        let test_cases_deser = [
            (
                "SECONDD_METHOD : LINEARRESPONSE",
                SeconddMethod::LinearResponse,
            ),
            (
                "SECONDD_METHOD : linearresponse",
                SeconddMethod::LinearResponse,
            ),
            (
                "SECONDD_METHOD : LINEARRESPONSE",
                SeconddMethod::LinearResponse,
            ), // Uppercase alias
            ("SECONDD_METHOD : DFPT", SeconddMethod::LinearResponse), // Alias
            ("SECONDD_METHOD : dfpt", SeconddMethod::LinearResponse), // Alias
            (
                "SECONDD_METHOD : FINITEDISPLACEMENT",
                SeconddMethod::FiniteDisplacement,
            ),
            (
                "SECONDD_METHOD : finitedisplacement",
                SeconddMethod::FiniteDisplacement,
            ),
            (
                "SECONDD_METHOD : FINITEDISPLACEMENT",
                SeconddMethod::FiniteDisplacement,
            ), // Uppercase alias
        ];

        for (input_str, expected_method) in test_cases_deser {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithSecondMethod {
                secondd_method: SeconddMethod,
            }

            let cell_file_result: Result<CellFileWithSecondMethod, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.secondd_method, expected_method,
                "Failed for input: {input_str}"
            );
        }

        // Test Serialization
        let second_method_instance = SeconddMethod::FiniteDisplacement;
        let serialized_result = to_string(&second_method_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized SECONDD_METHOD (FINITEDISPLACEMENT): {serialized_string}");
        assert!(serialized_string.contains("SECONDD_METHOD"));
        assert!(serialized_string.contains("FINITEDISPLACEMENT"));

        // Test Default
        assert_eq!(SeconddMethod::default(), SeconddMethod::LinearResponse);
    }
}
