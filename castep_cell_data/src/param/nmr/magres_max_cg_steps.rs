use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the maximum number of conjugate gradient steps during an NMR calculation.
///
/// Keyword type: Integer
///
/// Default: 250
///
/// Example:
/// MAGRES_MAX_CG_STEPS : 300
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MAGRES_MAX_CG_STEPS")]
pub struct MagresMaxCgSteps(pub u32); // Using u32 as it's a count of steps

impl Default for MagresMaxCgSteps {
    fn default() -> Self {
        Self(250) // Default is 250
    }
}

impl ToCell for MagresMaxCgSteps {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MAGRES_MAX_CG_STEPS", CellValue::UInt(self.0))
    }
}

impl ToCellValue for MagresMaxCgSteps {
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
    fn test_magres_max_cg_steps_serde() {
        let magres_max_cg_steps_str = "MAGRES_MAX_CG_STEPS : 300";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMagresMaxCgSteps {
            magres_max_cg_steps: MagresMaxCgSteps,
        }

        let cell_file_result: Result<CellFileWithMagresMaxCgSteps, _> =
            from_str(magres_max_cg_steps_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.magres_max_cg_steps.0, 300);

        let magres_max_cg_steps_instance = MagresMaxCgSteps(500);
        let serialized_result = to_string(&magres_max_cg_steps_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MAGRES_MAX_CG_STEPS (500): {serialized_string}");
        assert!(serialized_string.contains("MAGRES_MAX_CG_STEPS"));
        assert!(serialized_string.contains("500"));

        assert_eq!(MagresMaxCgSteps::default(), MagresMaxCgSteps(250));
    }
}
