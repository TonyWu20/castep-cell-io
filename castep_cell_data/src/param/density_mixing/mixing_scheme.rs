use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines which mixing scheme will be used in the density mixing procedure.
///
/// Keyword type: String
///
/// Default: MixingScheme::Broyden
///
/// Example:
/// MIXING_SCHEME : Pulay
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MIXING_SCHEME")]
pub enum MixingScheme {
    /// Kerker mixing scheme
    #[serde(alias = "kerker", alias = "KERKER")]
    Kerker,
    /// Linear mixing scheme
    #[serde(alias = "linear", alias = "LINEAR")]
    Linear,
    /// Broyden mixing scheme
    #[serde(alias = "broyden", alias = "BROYDEN")]
    Broyden,
    /// Pulay mixing scheme
    #[serde(alias = "pulay", alias = "PULAY")]
    Pulay,
}

impl Default for MixingScheme {
    fn default() -> Self {
        Self::Broyden // Default is Broyden
    }
}

impl ToCell for MixingScheme {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MIXING_SCHEME", self.to_cell_value())
    }
}

impl ToCellValue for MixingScheme {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                MixingScheme::Kerker => "Kerker",
                MixingScheme::Linear => "Linear",
                MixingScheme::Broyden => "Broyden",
                MixingScheme::Pulay => "Pulay",
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
    fn test_mixing_scheme_serde() {
        // Test Deserialization for various cases
        let test_cases_deser = [
            ("MIXING_SCHEME : Kerker", MixingScheme::Kerker),
            ("MIXING_SCHEME : kerker", MixingScheme::Kerker),
            ("MIXING_SCHEME : KERKER", MixingScheme::Kerker),
            ("MIXING_SCHEME : Linear", MixingScheme::Linear),
            ("MIXING_SCHEME : linear", MixingScheme::Linear),
            ("MIXING_SCHEME : LINEAR", MixingScheme::Linear),
            ("MIXING_SCHEME : Broyden", MixingScheme::Broyden),
            ("MIXING_SCHEME : broyden", MixingScheme::Broyden),
            ("MIXING_SCHEME : BROYDEN", MixingScheme::Broyden),
            ("MIXING_SCHEME : Pulay", MixingScheme::Pulay),
            ("MIXING_SCHEME : pulay", MixingScheme::Pulay),
            ("MIXING_SCHEME : PULAY", MixingScheme::Pulay),
        ];

        for (input_str, expected_scheme) in test_cases_deser {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithMixingScheme {
                mixing_scheme: MixingScheme,
            }

            let cell_file_result: Result<CellFileWithMixingScheme, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.mixing_scheme, expected_scheme,
                "Failed for input: {input_str}"
            );
        }

        // Test Serialization
        let mixing_scheme_instance = MixingScheme::Pulay;
        let serialized_result = to_string(&mixing_scheme_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MIXING_SCHEME (Pulay): {serialized_string}");
        assert!(serialized_string.contains("MIXING_SCHEME"));
        assert!(serialized_string.contains("Pulay"));

        // Test Default
        assert_eq!(MixingScheme::default(), MixingScheme::Broyden);
    }
}
