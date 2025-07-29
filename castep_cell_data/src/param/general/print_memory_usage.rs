use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies whether CASTEP should print memory estimates during the run.
///
/// Keyword type: Logical
///
/// Default: TRUE
///
/// Example:
/// PRINT_MEMORY_USAGE : FALSE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "PRINT_MEMORY_USAGE")]
pub struct PrintMemoryUsage(pub bool);

impl Default for PrintMemoryUsage {
    fn default() -> Self {
        Self(true) // Default is TRUE
    }
}

impl ToCell for PrintMemoryUsage {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PRINT_MEMORY_USAGE", CellValue::Bool(self.0))
    }
}

impl ToCellValue for PrintMemoryUsage {
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
    fn test_print_memory_usage_serde() {
        let print_memory_usage_true_str = "PRINT_MEMORY_USAGE : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPrintMemoryUsageTrue {
            print_memory_usage: PrintMemoryUsage,
        }

        let cell_file_true_result: Result<CellFileWithPrintMemoryUsageTrue, _> =
            from_str(print_memory_usage_true_str);
        assert!(
            cell_file_true_result.is_ok(),
            "Deserialization (TRUE) failed: {:?}",
            cell_file_true_result.err()
        );
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.print_memory_usage.0);

        let print_memory_usage_false_str = "PRINT_MEMORY_USAGE : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPrintMemoryUsageFalse {
            print_memory_usage: PrintMemoryUsage,
        }

        let cell_file_false_result: Result<CellFileWithPrintMemoryUsageFalse, _> =
            from_str(print_memory_usage_false_str);
        assert!(
            cell_file_false_result.is_ok(),
            "Deserialization (FALSE) failed: {:?}",
            cell_file_false_result.err()
        );
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.print_memory_usage.0);

        let print_memory_usage_instance = PrintMemoryUsage(false);
        let serialized_result = to_string(&print_memory_usage_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized PRINT_MEMORY_USAGE (FALSE):\n{serialized_string}");
        assert!(serialized_string.contains("PRINT_MEMORY_USAGE"));
        assert!(serialized_string.contains("false") || serialized_string.contains("FALSE"));

        assert_eq!(PrintMemoryUsage::default(), PrintMemoryUsage(true));
    }
}
