use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the name of file to which checkpoint (continuation) data are to be written.
///
/// Keyword type: String
///
/// Default: seedname.check
///
/// Example:
/// CHECKPOINT : test.check
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "CHECKPOINT")]
pub struct Checkpoint(pub String);

impl ToCell for Checkpoint {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("CHECKPOINT", CellValue::String(self.0.clone()))
    }
}

impl ToCellValue for Checkpoint {
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
    fn test_checkpoint_serde() {
        // 1. Test Deserialization
        let checkpoint_str = "CHECKPOINT : test.check";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithCheckpoint {
            checkpoint: Checkpoint,
        }

        let cell_file_result: Result<CellFileWithCheckpoint, _> = from_str(checkpoint_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.checkpoint.0, "test.check");

        // 2. Test Serialization using ToCell
        let checkpoint_instance = Checkpoint("my_run.chk".to_string());
        let serialized_result = to_string(&checkpoint_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        println!("Serialized CHECKPOINT (my_run.chk):\n{serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("CHECKPOINT"));
        assert!(serialized_string.contains("my_run.chk"));
    }
}
