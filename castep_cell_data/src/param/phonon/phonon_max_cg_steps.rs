use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the maximum number of conjugate gradient steps during a phonon calculation.
///
/// Keyword type: Integer
///
/// Default: 20
///
/// Example:
/// PHONON_MAX_CG_STEPS : 10
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "PHONON_MAX_CG_STEPS")]
pub struct PhononMaxCgSteps(pub u32); // Using i32

impl Default for PhononMaxCgSteps {
    fn default() -> Self {
        Self(20) // Default is 20
    }
}

impl ToCell for PhononMaxCgSteps {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PHONON_MAX_CG_STEPS", CellValue::UInt(self.0))
    }
}

impl ToCellValue for PhononMaxCgSteps {
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
    fn test_phonon_max_cg_steps_serde() {
        let phonon_max_cg_steps_str = "PHONON_MAX_CG_STEPS : 10";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPhononMaxCgSteps {
            phonon_max_cg_steps: PhononMaxCgSteps,
        }

        let cell_file_result: Result<CellFileWithPhononMaxCgSteps, _> =
            from_str(phonon_max_cg_steps_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.phonon_max_cg_steps.0, 10);

        let phonon_max_cg_steps_instance = PhononMaxCgSteps(15);
        let serialized_result = to_string(&phonon_max_cg_steps_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized PHONON_MAX_CG_STEPS (15): {serialized_string}");
        assert!(serialized_string.contains("PHONON_MAX_CG_STEPS"));
        assert!(serialized_string.contains("15"));

        assert_eq!(PhononMaxCgSteps::default(), PhononMaxCgSteps(20));
    }
}
