use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the method used to minimize electronic states.
///
/// Keyword type: String
///
/// Default: ElectronicMinimizer::Cg
///
/// Example:
/// ELECTRONIC_MINIMIZER : SD
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "ELECTRONIC_MINIMIZER")]
pub enum ElectronicMinimizer {
    /// Minimizer takes up to 10 SD steps
    #[serde(rename = "SD", alias = "sd")]
    Sd,
    /// Minimizer takes one SD step followed by up to 4 CG steps
    #[serde(rename = "CG", alias = "cg")]
    Cg,
    #[serde(rename = "RMM/DIIS", alias = "rmm/diis")]
    RmmDiis,
    // Note: RMM/DIIS is mentioned in defaults for MAX_SD_STEPS/MAX_CG_STEPS
    // but not listed as an option in the main description. If it's a valid option,
    // it should be added here.
}

impl Default for ElectronicMinimizer {
    fn default() -> Self {
        Self::Cg // Default is CG
    }
}

impl ToCell for ElectronicMinimizer {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("ELECTRONIC_MINIMIZER", self.to_cell_value())
    }
}

impl ToCellValue for ElectronicMinimizer {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                ElectronicMinimizer::Sd => "SD",
                ElectronicMinimizer::Cg => "CG",
                ElectronicMinimizer::RmmDiis => "RMM/DIIS", // If added
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
    fn test_electronic_minimizer_serde() {
        let test_cases = [
            ("ELECTRONIC_MINIMIZER : SD", ElectronicMinimizer::Sd),
            ("ELECTRONIC_MINIMIZER : sd", ElectronicMinimizer::Sd),
            ("electronic_minimizer : CG", ElectronicMinimizer::Cg),
            ("electronic_minimizer : cg", ElectronicMinimizer::Cg),
            // ("ELECTRONIC_MINIMIZER : RMM/DIIS", ElectronicMinimizer::RmmDiis), // If added
        ];

        for (input_str, expected_minimizer) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithElectronicMinimizer {
                #[serde(alias = "electronic_minimizer")]
                electronic_minimizer: ElectronicMinimizer,
            }

            let cell_file_result: Result<CellFileWithElectronicMinimizer, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.electronic_minimizer, expected_minimizer,
                "Failed for input: {input_str}"
            );
        }

        let electronic_minimizer_instance = ElectronicMinimizer::Sd;
        let serialized_result = to_string(&electronic_minimizer_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized ELECTRONIC_MINIMIZER (SD): {serialized_string}");
        assert!(serialized_string.contains("ELECTRONIC_MINIMIZER"));
        assert!(serialized_string.contains("SD"));

        assert_eq!(ElectronicMinimizer::default(), ElectronicMinimizer::Cg);
    }
}
