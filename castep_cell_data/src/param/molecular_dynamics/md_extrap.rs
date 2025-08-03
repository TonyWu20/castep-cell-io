use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the wavefunction extrapolation scheme used for MD.
///
/// Keyword type: String
///
/// Default: MdExtrap::First
///
/// Example:
/// MD_EXTRAP : Mixed
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MD_EXTRAP")]
pub enum MdExtrap {
    /// No extrapolation used
    #[serde(alias = "none", alias = "NONE")]
    None,
    /// First order extrapolation
    #[serde(alias = "first", alias = "FIRST")]
    First,
    /// Second order extrapolation
    #[serde(alias = "second", alias = "SECOND")]
    Second,
    /// Alternating first and second order extrapolation
    #[serde(alias = "mixed", alias = "MIXED")]
    Mixed,
}

impl Default for MdExtrap {
    fn default() -> Self {
        Self::First // Default is First
    }
}

impl ToCell for MdExtrap {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_EXTRAP", self.to_cell_value())
    }
}

impl ToCellValue for MdExtrap {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                MdExtrap::None => "None",
                MdExtrap::First => "First",
                MdExtrap::Second => "Second",
                MdExtrap::Mixed => "Mixed",
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
    fn test_md_extrap_serde() {
        // Test Deserialization for various cases
        let test_cases_deser = [
            ("MD_EXTRAP : None", MdExtrap::None),
            ("MD_EXTRAP : none", MdExtrap::None),
            ("MD_EXTRAP : NONE", MdExtrap::None), // Uppercase alias
            ("MD_EXTRAP : First", MdExtrap::First),
            ("MD_EXTRAP : first", MdExtrap::First),
            ("MD_EXTRAP : FIRST", MdExtrap::First), // Uppercase alias
            ("MD_EXTRAP : Second", MdExtrap::Second),
            ("MD_EXTRAP : second", MdExtrap::Second),
            ("MD_EXTRAP : SECOND", MdExtrap::Second), // Uppercase alias
            ("MD_EXTRAP : Mixed", MdExtrap::Mixed),
            ("MD_EXTRAP : mixed", MdExtrap::Mixed),
            ("MD_EXTRAP : MIXED", MdExtrap::Mixed), // Uppercase alias
        ];

        for (input_str, expected_extrap) in test_cases_deser {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithMdExtrap {
                md_extrap: MdExtrap,
            }

            let cell_file_result: Result<CellFileWithMdExtrap, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.md_extrap, expected_extrap,
                "Failed for input: {input_str}"
            );
        }

        // Test Serialization
        let md_extrap_instance = MdExtrap::Mixed;
        let serialized_result = to_string(&md_extrap_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MD_EXTRAP (Mixed): {serialized_string}");
        assert!(serialized_string.contains("MD_EXTRAP"));
        assert!(serialized_string.contains("Mixed"));

        // Test Default
        assert_eq!(MdExtrap::default(), MdExtrap::First);
    }
}
