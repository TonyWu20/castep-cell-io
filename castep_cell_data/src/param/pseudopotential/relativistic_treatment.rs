// File: general/relativistic_treatment.rs
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Selects the method used to treat relativistic effects.
///
/// Keyword type: String
///
/// Default: RelativisticTreatment::KoellingHarmon
///
/// Example:
/// RELATIVISTIC_TREATMENT : ZORA
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "RELATIVISTIC_TREATMENT")]
pub enum RelativisticTreatment {
    /// Completely non-relativistic pseudopotentials
    #[serde(rename = "SCHROEDINGER")]
    Schroedinger,
    /// Scalar relativistic treatment (ZORA)
    #[serde(rename = "ZORA")]
    Zora,
    /// Scalar relativistic treatment (Koelling-Harmon)
    #[serde(rename = "KOELLING-HARMON")]
    #[default]
    KoellingHarmon,
    /// Fully relativistic treatment
    #[serde(rename = "DIRAC")]
    Dirac,
}

impl ToCell for RelativisticTreatment {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("RELATIVISTIC_TREATMENT", self.to_cell_value())
    }
}

impl ToCellValue for RelativisticTreatment {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                RelativisticTreatment::Schroedinger => "SCHROEDINGER",
                RelativisticTreatment::Zora => "ZORA",
                RelativisticTreatment::KoellingHarmon => "KOELLING-HARMON",
                RelativisticTreatment::Dirac => "DIRAC",
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
    fn test_relativistic_treatment_serde() {
        // 1. Test Deserialization for variants
        let test_cases = [
            (
                "RELATIVISTIC_TREATMENT : SCHROEDINGER",
                RelativisticTreatment::Schroedinger,
            ),
            ("RELATIVISTIC_TREATMENT : ZORA", RelativisticTreatment::Zora),
            (
                "RELATIVISTIC_TREATMENT : KOELLING-HARMON",
                RelativisticTreatment::KoellingHarmon,
            ),
            (
                "RELATIVISTIC_TREATMENT : DIRAC",
                RelativisticTreatment::Dirac,
            ),
        ];

        for (input_str, expected_treatment) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithRelativisticTreatment {
                relativistic_treatment: RelativisticTreatment,
            }

            let cell_file_result: Result<CellFileWithRelativisticTreatment, _> =
                from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.relativistic_treatment, expected_treatment,
                "Failed for input: {}",
                input_str
            );
        }

        // 2. Test Serialization using ToCell
        let relativistic_treatment_instance = RelativisticTreatment::Zora;
        let serialized_result = to_string(&relativistic_treatment_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized RELATIVISTIC_TREATMENT (ZORA): {serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("RELATIVISTIC_TREATMENT"));
        assert!(serialized_string.contains("ZORA"));

        // 3. Test Default
        assert_eq!(
            RelativisticTreatment::default(),
            RelativisticTreatment::KoellingHarmon
        );
    }
}
