use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the maximum number of densities to store in the history used during the density mixing procedure.
///
/// Keyword type: Integer
///
/// Default: 7
///
/// Example:
/// MIX_HISTORY_LENGTH : 5
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MIX_HISTORY_LENGTH")]
pub struct MixHistoryLength(pub i32); // Using i32 for potential negative checks if needed; To be confirmed

impl Default for MixHistoryLength {
    fn default() -> Self {
        Self(7) // Default is 7
    }
}

impl ToCell for MixHistoryLength {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MIX_HISTORY_LENGTH", CellValue::Int(self.0))
    }
}

impl ToCellValue for MixHistoryLength {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_mix_history_length_serde() {
        let mix_history_length_str = "MIX_HISTORY_LENGTH : 5";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMixHistoryLength {
            mix_history_length: MixHistoryLength,
        }

        let cell_file_result: Result<CellFileWithMixHistoryLength, _> =
            from_str(mix_history_length_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.mix_history_length.0, 5);

        let mix_history_length_instance = MixHistoryLength(10);
        let serialized_result = to_string(&mix_history_length_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MIX_HISTORY_LENGTH (10): {serialized_string}");
        assert!(serialized_string.contains("MIX_HISTORY_LENGTH"));
        assert!(serialized_string.contains("10"));

        assert_eq!(MixHistoryLength::default(), MixHistoryLength(7));
    }
}
