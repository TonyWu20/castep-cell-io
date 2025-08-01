use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the maximum number of conjugate gradient steps in an SCF cycle.
///
/// Keyword type: Integer
///
/// Default:
/// SD then MAX_CG_STEPS : 0
/// CG then MAX_CG_STEPS : 4
/// RMM/DIIS then MAX_CG_STEPS : 2
/// If ELECTRONIC_MINIMIZER is not defined, the default is 4.
///
/// Example:
/// MAX_CG_STEPS : 5
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MAX_CG_STEPS")]
pub struct MaxCgSteps(pub i32);

// Note: Default logic is context-dependent (depends on ELECTRONIC_MINIMIZER).
// The `Default` implementation here provides a base default.
impl Default for MaxCgSteps {
    fn default() -> Self {
        Self(4) // Base default is 4 (if ELECTRONIC_MINIMIZER is not defined)
    }
}

impl ToCell for MaxCgSteps {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MAX_CG_STEPS", CellValue::Int(self.0))
    }
}

impl ToCellValue for MaxCgSteps {
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
    fn test_max_cg_steps_serde() {
        let max_cg_steps_str = "MAX_CG_STEPS : 5";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMaxCgSteps {
            max_cg_steps: MaxCgSteps,
        }

        let cell_file_result: Result<CellFileWithMaxCgSteps, _> = from_str(max_cg_steps_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.max_cg_steps.0, 5);

        let max_cg_steps_instance = MaxCgSteps(2);
        let serialized_result = to_string(&max_cg_steps_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MAX_CG_STEPS (2): {serialized_string}");
        assert!(serialized_string.contains("MAX_CG_STEPS"));
        assert!(serialized_string.contains("2"));

        assert_eq!(MaxCgSteps::default(), MaxCgSteps(4));
    }
}
