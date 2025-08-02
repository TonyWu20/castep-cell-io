use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the verbosity of reporting of population analysis results.
///
/// Keyword type: String
///
/// Default: PopnWrite::Enhanced
///
/// Example:
/// POPN_WRITE : SUMMARY
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "POPN_WRITE")]
pub enum PopnWrite {
    /// No output
    #[serde(alias = "none", alias = "NONE")]
    None,
    /// Summary only
    #[serde(alias = "minimal", alias = "MINIMAL")]
    Minimal,
    /// Same as MINIMAL
    #[serde(alias = "summary", alias = "SUMMARY")]
    Summary,
    /// Summary and orbital-resolved PDOS components
    #[serde(alias = "enhanced", alias = "ENHANCED")]
    #[default]
    Enhanced,
    /// As ENHANCED and S and T matrices
    #[serde(alias = "verbose", alias = "VERBOSE")]
    Verbose,
}

impl ToCell for PopnWrite {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("POPN_WRITE", self.to_cell_value())
    }
}

impl ToCellValue for PopnWrite {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                PopnWrite::None => "NONE",
                PopnWrite::Minimal => "MINIMAL",
                PopnWrite::Summary => "SUMMARY",
                PopnWrite::Enhanced => "ENHANCED",
                PopnWrite::Verbose => "VERBOSE",
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
    fn test_popn_write_serde() {
        // Test Deserialization for various cases
        let test_cases_deser = [
            ("POPN_WRITE : NONE", PopnWrite::None),
            ("POPN_WRITE : none", PopnWrite::None),
            ("POPN_WRITE : NONE", PopnWrite::None), // Uppercase alias
            ("POPN_WRITE : MINIMAL", PopnWrite::Minimal),
            ("POPN_WRITE : minimal", PopnWrite::Minimal),
            ("POPN_WRITE : MINIMAL", PopnWrite::Minimal), // Uppercase alias
            ("POPN_WRITE : SUMMARY", PopnWrite::Summary),
            ("POPN_WRITE : summary", PopnWrite::Summary),
            ("POPN_WRITE : SUMMARY", PopnWrite::Summary), // Uppercase alias
            ("POPN_WRITE : ENHANCED", PopnWrite::Enhanced),
            ("POPN_WRITE : enhanced", PopnWrite::Enhanced),
            ("POPN_WRITE : ENHANCED", PopnWrite::Enhanced), // Uppercase alias
            ("POPN_WRITE : VERBOSE", PopnWrite::Verbose),
            ("POPN_WRITE : verbose", PopnWrite::Verbose),
            ("POPN_WRITE : VERBOSE", PopnWrite::Verbose), // Uppercase alias
        ];

        for (input_str, expected_write) in test_cases_deser {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithPopnWrite {
                popn_write: PopnWrite,
            }

            let cell_file_result: Result<CellFileWithPopnWrite, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.popn_write, expected_write,
                "Failed for input: {input_str}"
            );
        }

        // Test Serialization
        let popn_write_instance = PopnWrite::Verbose;
        let serialized_result = to_string(&popn_write_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized POPN_WRITE (VERBOSE): {serialized_string}");
        assert!(serialized_string.contains("POPN_WRITE"));
        assert!(serialized_string.contains("VERBOSE"));

        // Test Default
        assert_eq!(PopnWrite::default(), PopnWrite::Enhanced);
    }
}
