use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the maximum number of conjugate gradient steps taken for each electronic band.
///
/// Keyword type: Integer
///
/// Default: 4
///
/// Example:
/// BS_MAX_CG_STEPS : 10
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "BS_MAX_CG_STEPS")]
pub struct BsMaxCgSteps(pub u32); // Using i32

impl Default for BsMaxCgSteps {
    fn default() -> Self {
        Self(4) // Default is 4
    }
}

impl ToCell for BsMaxCgSteps {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("BS_MAX_CG_STEPS", CellValue::UInt(self.0))
    }
}

impl ToCellValue for BsMaxCgSteps {
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
    fn test_bs_max_cg_steps_serde() {
        let bs_max_cg_steps_str = "BS_MAX_CG_STEPS : 10";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithBsMaxCgSteps {
            bs_max_cg_steps: BsMaxCgSteps,
        }

        let cell_file_result: Result<CellFileWithBsMaxCgSteps, _> = from_str(bs_max_cg_steps_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.bs_max_cg_steps.0, 10);

        let bs_max_cg_steps_instance = BsMaxCgSteps(8);
        let serialized_result = to_string(&bs_max_cg_steps_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized BS_MAX_CG_STEPS (8): {serialized_string}");
        assert!(serialized_string.contains("BS_MAX_CG_STEPS"));
        assert!(serialized_string.contains("8"));

        assert_eq!(BsMaxCgSteps::default(), BsMaxCgSteps(4));
    }
}
