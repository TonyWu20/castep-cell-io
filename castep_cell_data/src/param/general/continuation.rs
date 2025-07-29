use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the model file used to continue the job.
///
/// Keyword type: String
///
/// Default: NULL (no continuation)
///
/// Example:
/// CONTINUATION : DEFAULT
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "CONTINUATION")]
pub struct Continuation(pub String); // Could potentially be an enum for NULL/DEFAULT/values

impl ToCell for Continuation {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("CONTINUATION", CellValue::String(self.0.clone()))
    }
}

impl ToCellValue for Continuation {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_continuation_serde() {
        // 1. Test Deserialization DEFAULT
        let continuation_default_str = "CONTINUATION : DEFAULT";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithContinuationDefault {
            continuation: Continuation,
        }

        let cell_file_default_result: Result<CellFileWithContinuationDefault, _> =
            from_str(continuation_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (DEFAULT) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert_eq!(cell_file_default.continuation.0, "DEFAULT");

        // 2. Test Deserialization filename
        let continuation_file_str = "CONTINUATION : previous_run.check";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithContinuationFile {
            continuation: Continuation,
        }

        let cell_file_file_result: Result<CellFileWithContinuationFile, _> =
            from_str(continuation_file_str);
        assert!(
            cell_file_file_result.is_ok(),
            "Deserialization (file) failed: {:?}",
            cell_file_file_result.err()
        );
        let cell_file_file = cell_file_file_result.unwrap();
        assert_eq!(cell_file_file.continuation.0, "previous_run.check");

        // 3. Test Serialization using ToCell
        let continuation_instance = Continuation("old_calc.chk".to_string());
        let serialized_result = to_string(&continuation_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        println!("Serialized CONTINUATION (old_calc.chk):\n{serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("CONTINUATION"));
        assert!(serialized_string.contains("old_calc.chk"));
    }
}
