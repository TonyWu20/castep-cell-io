use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the damping scheme used during damped molecular dynamics.
///
/// Keyword type: String
///
/// Default: MdDampingScheme::Independent
///
/// Example:
/// MD_DAMPING_SCHEME : Coupled
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MD_DAMPING_SCHEME")]
pub enum MdDampingScheme {
    /// Calculates optimal damping parameters according to the independent modes algorithm
    #[serde(alias = "independent", alias = "INDEPENDENT")]
    Independent,
    /// Calculates optimal damping parameters according to the coupled modes algorithm
    #[serde(alias = "coupled", alias = "COUPLED")]
    Coupled,
    /// Performs steepest descent dynamics
    #[serde(alias = "steepestdescents", alias = "STEEPESTDESCENTS")]
    SteepestDescents,
}

impl Default for MdDampingScheme {
    fn default() -> Self {
        Self::Independent // Default is Independent
    }
}

impl ToCell for MdDampingScheme {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_DAMPING_SCHEME", self.to_cell_value())
    }
}

impl ToCellValue for MdDampingScheme {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                MdDampingScheme::Independent => "Independent",
                MdDampingScheme::Coupled => "Coupled",
                MdDampingScheme::SteepestDescents => "SteepestDescents",
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
    fn test_md_damping_scheme_serde() {
        // Test Deserialization for various cases
        let test_cases_deser = [
            (
                "MD_DAMPING_SCHEME : Independent",
                MdDampingScheme::Independent,
            ),
            (
                "MD_DAMPING_SCHEME : independent",
                MdDampingScheme::Independent,
            ),
            (
                "MD_DAMPING_SCHEME : INDEPENDENT",
                MdDampingScheme::Independent,
            ), // Uppercase alias
            ("MD_DAMPING_SCHEME : Coupled", MdDampingScheme::Coupled),
            ("MD_DAMPING_SCHEME : coupled", MdDampingScheme::Coupled),
            ("MD_DAMPING_SCHEME : COUPLED", MdDampingScheme::Coupled), // Uppercase alias
            (
                "MD_DAMPING_SCHEME : SteepestDescents",
                MdDampingScheme::SteepestDescents,
            ),
            (
                "MD_DAMPING_SCHEME : steepestdescents",
                MdDampingScheme::SteepestDescents,
            ),
            (
                "MD_DAMPING_SCHEME : STEEPESTDESCENTS",
                MdDampingScheme::SteepestDescents,
            ), // Uppercase alias
        ];

        for (input_str, expected_scheme) in test_cases_deser {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithMdDampingScheme {
                md_damping_scheme: MdDampingScheme,
            }

            let cell_file_result: Result<CellFileWithMdDampingScheme, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.md_damping_scheme, expected_scheme,
                "Failed for input: {input_str}"
            );
        }

        // Test Serialization
        let md_damping_scheme_instance = MdDampingScheme::Coupled;
        let serialized_result = to_string(&md_damping_scheme_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MD_DAMPING_SCHEME (Coupled): {serialized_string}");
        assert!(serialized_string.contains("MD_DAMPING_SCHEME"));
        assert!(serialized_string.contains("Coupled"));

        // Test Default
        assert_eq!(MdDampingScheme::default(), MdDampingScheme::Independent);
    }
}
