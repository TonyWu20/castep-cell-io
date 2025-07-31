use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the total number of down-spin electrons.
/// Used only if SPIN_POLARIZED = TRUE.
///
/// Keyword type: Real
///
/// Default:
/// If SPIN is specified: NDOWN = (NELECTRONS - SPIN) / 2
/// Else: NDOWN = NELECTRONS / 2
///
/// Example:
/// NDOWN : 12.0
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "NDOWN")]
pub struct Ndown(pub f64);

// Note: Default logic is context-dependent.
// The `Default` implementation here is omitted as it's not directly applicable.
// A containing struct or the application logic would need to handle this.

impl ToCell for Ndown {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("NDOWN", CellValue::Float(self.0))
    }
}

impl ToCellValue for Ndown {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Float(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_ndown_serde() {
        let ndown_str = "NDOWN : 12.0";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithNdown {
            ndown: Ndown,
        }

        let cell_file_result: Result<CellFileWithNdown, _> = from_str(ndown_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.ndown.0 - 12.0).abs() < f64::EPSILON);

        let ndown_instance = Ndown(8.5);
        let serialized_result = to_string(&ndown_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized NDOWN (8.5): {serialized_string}");
        assert!(serialized_string.contains("NDOWN"));
        assert!(serialized_string.contains("8.5"));
    }
}
