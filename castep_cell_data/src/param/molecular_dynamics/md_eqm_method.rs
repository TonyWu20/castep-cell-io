use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the scheme to be used for enhanced MD equilibration.
///
/// Keyword type: String
///
/// Default: MdEqmMethod::None
///
/// Example:
/// MD_EQM_METHOD : BERENDSEN
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MD_EQM_METHOD")]
pub enum MdEqmMethod {
    /// No enhanced equilibration
    #[serde(alias = "none", alias = "NONE")]
    None,
    /// Berendsen weak coupling scheme
    #[serde(alias = "berendsen", alias = "BERENDSEN")]
    Berendsen,
}

impl Default for MdEqmMethod {
    fn default() -> Self {
        Self::None // Default is NONE
    }
}

impl ToCell for MdEqmMethod {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_EQM_METHOD", self.to_cell_value())
    }
}

impl ToCellValue for MdEqmMethod {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                MdEqmMethod::None => "NONE",
                MdEqmMethod::Berendsen => "BERENDSEN",
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
    fn test_md_eqm_method_serde() {
        // Test Deserialization for various cases
        let test_cases_deser = [
            ("MD_EQM_METHOD : NONE", MdEqmMethod::None),
            ("MD_EQM_METHOD : none", MdEqmMethod::None),
            ("MD_EQM_METHOD : NONE", MdEqmMethod::None), // Uppercase alias
            ("MD_EQM_METHOD : BERENDSEN", MdEqmMethod::Berendsen),
            ("MD_EQM_METHOD : berendsen", MdEqmMethod::Berendsen),
            ("MD_EQM_METHOD : BERENDSEN", MdEqmMethod::Berendsen), // Uppercase alias
        ];

        for (input_str, expected_method) in test_cases_deser {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithMdEqmMethod {
                md_eqm_method: MdEqmMethod,
            }

            let cell_file_result: Result<CellFileWithMdEqmMethod, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.md_eqm_method, expected_method,
                "Failed for input: {input_str}"
            );
        }

        // Test Serialization
        let md_eqm_method_instance = MdEqmMethod::Berendsen;
        let serialized_result = to_string(&md_eqm_method_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MD_EQM_METHOD (BERENDSEN): {serialized_string}");
        assert!(serialized_string.contains("MD_EQM_METHOD"));
        assert!(serialized_string.contains("BERENDSEN"));

        // Test Default
        assert_eq!(MdEqmMethod::default(), MdEqmMethod::None);
    }
}
