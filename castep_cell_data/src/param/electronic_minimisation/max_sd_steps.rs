use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the maximum number of steepest descent steps in an SCF cycle.
///
/// Keyword type: Integer
///
/// Default:
/// SD then MAX_SD_STEPS : 10
/// CG then MAX_SD_STEPS : 1
/// RMM/DIIS then MAX_SD_STEPS : 1
/// If ELECTRONIC_MINIMIZER is not defined, the default is 1.
///
/// Example:
/// MAX_SD_STEPS : 5
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MAX_SD_STEPS")]
pub struct MaxSdSteps(pub i32);

// Note: Default logic is context-dependent (depends on ELECTRONIC_MINIMIZER).
// The `Default` implementation here provides a base default.
impl Default for MaxSdSteps {
    fn default() -> Self {
        Self(1) // Base default is 1 (if ELECTRONIC_MINIMIZER is not defined)
    }
}

impl ToCell for MaxSdSteps {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MAX_SD_STEPS", CellValue::Int(self.0))
    }
}

impl ToCellValue for MaxSdSteps {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_max_sd_steps_serde() {
        let max_sd_steps_str = "MAX_SD_STEPS : 5";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMaxSdSteps {
            max_sd_steps: MaxSdSteps,
        }

        let cell_file_result: Result<CellFileWithMaxSdSteps, _> = from_str(max_sd_steps_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.max_sd_steps.0, 5);

        let max_sd_steps_instance = MaxSdSteps(10);
        let serialized_result = to_string(&max_sd_steps_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MAX_SD_STEPS (10): {serialized_string}");
        assert!(serialized_string.contains("MAX_SD_STEPS"));
        assert!(serialized_string.contains("10"));

        assert_eq!(MaxSdSteps::default(), MaxSdSteps(1));
    }
}
