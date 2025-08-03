use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the maximum number of conjugate gradient steps during a linear response calculation.
///
/// Keyword type: Integer
///
/// Default: 20
///
/// Example:
/// EFIELD_MAX_CG_STEPS : 30
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "EFIELD_MAX_CG_STEPS")]
pub struct EfieldMaxCgSteps(pub u32); // Using u32 as it's a count of steps

impl Default for EfieldMaxCgSteps {
    fn default() -> Self {
        Self(20) // Default is 20
    }
}

impl ToCell for EfieldMaxCgSteps {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("EFIELD_MAX_CG_STEPS", CellValue::UInt(self.0))
    }
}

impl ToCellValue for EfieldMaxCgSteps {
    fn to_cell_value(&self) -> CellValue {
        CellValue::UInt(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_efield_max_cg_steps_serde() {
        let efield_max_cg_steps_str = "EFIELD_MAX_CG_STEPS : 30";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithEfieldMaxCgSteps {
            efield_max_cg_steps: EfieldMaxCgSteps,
        }

        let cell_file_result: Result<CellFileWithEfieldMaxCgSteps, _> =
            from_str(efield_max_cg_steps_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.efield_max_cg_steps.0, 30);

        let efield_max_cg_steps_instance = EfieldMaxCgSteps(40);
        let serialized_result = to_string(&efield_max_cg_steps_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized EFIELD_MAX_CG_STEPS (40): {serialized_string}");
        assert!(serialized_string.contains("EFIELD_MAX_CG_STEPS"));
        assert!(serialized_string.contains("40"));

        assert_eq!(EfieldMaxCgSteps::default(), EfieldMaxCgSteps(20));
    }
}
