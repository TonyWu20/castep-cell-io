use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the model file from which data will be read to initialize a new calculation.
///
/// Keyword type: String
///
/// Default: NULL (no reuse)
///
/// Example:
/// REUSE : DEFAULT
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "REUSE")]
pub struct Reuse(pub String); // Could potentially be an enum for NULL/DEFAULT/values

impl ToCell for Reuse {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("REUSE", CellValue::String(self.0.clone()))
    }
}

impl ToCellValue for Reuse {
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
    fn test_reuse_serde() {
        let reuse_default_str = "REUSE : DEFAULT";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithReuseDefault {
            reuse: Reuse,
        }

        let cell_file_default_result: Result<CellFileWithReuseDefault, _> =
            from_str(reuse_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (DEFAULT) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert_eq!(cell_file_default.reuse.0, "DEFAULT");

        let reuse_file_str = "REUSE : previous_run.check";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithReuseFile {
            reuse: Reuse,
        }

        let cell_file_file_result: Result<CellFileWithReuseFile, _> = from_str(reuse_file_str);
        assert!(
            cell_file_file_result.is_ok(),
            "Deserialization (file) failed: {:?}",
            cell_file_file_result.err()
        );
        let cell_file_file = cell_file_file_result.unwrap();
        assert_eq!(cell_file_file.reuse.0, "previous_run.check");

        let reuse_instance = Reuse("old_calc.chk".to_string());
        let serialized_result = to_string(&reuse_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized REUSE (old_calc.chk):\n{serialized_string}");
        assert!(serialized_string.contains("REUSE"));
        assert!(serialized_string.contains("old_calc.chk"));
    }
}
