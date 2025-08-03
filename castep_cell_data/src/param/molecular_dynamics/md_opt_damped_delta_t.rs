use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls whether or not the optimal time step will be calculated after each damped MD step.
///
/// Keyword type: Logical
///
/// Default: false
///
/// Example:
/// MD_OPT_DAMPED_DELTA_T : TRUE
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MD_OPT_DAMPED_DELTA_T")]
pub struct MdOptDampedDeltaT(pub bool);

impl ToCell for MdOptDampedDeltaT {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_OPT_DAMPED_DELTA_T", CellValue::Bool(self.0))
    }
}

impl ToCellValue for MdOptDampedDeltaT {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_md_opt_damped_delta_t_serde() {
        // 1. Test Deserialization TRUE
        let md_opt_damped_delta_t_true_str = "MD_OPT_DAMPED_DELTA_T : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMdOptDampedDeltaTTrue {
            md_opt_damped_delta_t: MdOptDampedDeltaT,
        }

        let cell_file_true_result: Result<CellFileWithMdOptDampedDeltaTTrue, _> =
            from_str(md_opt_damped_delta_t_true_str);
        assert!(
            cell_file_true_result.is_ok(),
            "Deserialization (TRUE) failed: {:?}",
            cell_file_true_result.err()
        );
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.md_opt_damped_delta_t.0);

        // 2. Test Deserialization FALSE
        let md_opt_damped_delta_t_false_str = "MD_OPT_DAMPED_DELTA_T : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMdOptDampedDeltaTFalse {
            md_opt_damped_delta_t: MdOptDampedDeltaT,
        }

        let cell_file_false_result: Result<CellFileWithMdOptDampedDeltaTFalse, _> =
            from_str(md_opt_damped_delta_t_false_str);
        assert!(
            cell_file_false_result.is_ok(),
            "Deserialization (FALSE) failed: {:?}",
            cell_file_false_result.err()
        );
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.md_opt_damped_delta_t.0);

        // 3. Test Serialization using ToCell
        let md_opt_damped_delta_t_instance = MdOptDampedDeltaT(true);
        let serialized_result = to_string(&md_opt_damped_delta_t_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MD_OPT_DAMPED_DELTA_T (TRUE): {serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("MD_OPT_DAMPED_DELTA_T"));
        assert!(serialized_string.contains("true") || serialized_string.contains("TRUE"));

        // 4. Test Default
        assert_eq!(MdOptDampedDeltaT::default(), MdOptDampedDeltaT(false));
    }
}
