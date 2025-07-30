use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies whether the final electron density is written to an ASCII file.
///
/// Keyword type: Logical
///
/// Default: FALSE
///
/// Example:
/// WRITE_FORMATTED_DENSITY : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "WRITE_FORMATTED_DENSITY")]
pub struct WriteFormattedDensity(pub bool);

impl Default for WriteFormattedDensity {
    fn default() -> Self {
        Self(false) // Default is FALSE
    }
}

impl ToCell for WriteFormattedDensity {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("WRITE_FORMATTED_DENSITY", CellValue::Bool(self.0))
    }
}

impl ToCellValue for WriteFormattedDensity {
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
    fn test_write_formatted_density_serde() {
        // 1. Test Deserialization TRUE
        let write_formatted_density_true_str = "WRITE_FORMATTED_DENSITY : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithWriteDensityTrue {
            write_formatted_density: WriteFormattedDensity,
        }

        let cell_file_true_result: Result<CellFileWithWriteDensityTrue, _> =
            from_str(write_formatted_density_true_str);
        assert!(
            cell_file_true_result.is_ok(),
            "Deserialization (TRUE) failed: {:?}",
            cell_file_true_result.err()
        );
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.write_formatted_density.0);

        // 2. Test Deserialization FALSE
        let write_formatted_density_false_str = "WRITE_FORMATTED_DENSITY : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithWriteDensityFalse {
            write_formatted_density: WriteFormattedDensity,
        }

        let cell_file_false_result: Result<CellFileWithWriteDensityFalse, _> =
            from_str(write_formatted_density_false_str);
        assert!(
            cell_file_false_result.is_ok(),
            "Deserialization (FALSE) failed: {:?}",
            cell_file_false_result.err()
        );
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.write_formatted_density.0);

        // 3. Test Serialization using ToCell
        let write_formatted_density_instance = WriteFormattedDensity(true);
        let serialized_result = to_string(&write_formatted_density_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized WRITE_FORMATTED_DENSITY (TRUE):\n{serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("WRITE_FORMATTED_DENSITY"));
        assert!(serialized_string.contains("true") || serialized_string.contains("TRUE"));

        // 4. Test Default
        assert_eq!(
            WriteFormattedDensity::default(),
            WriteFormattedDensity(false)
        );
    }
}
