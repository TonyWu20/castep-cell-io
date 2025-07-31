use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the total number of up-spin electrons.
/// Used only if SPIN_POLARIZED = TRUE.
///
/// Keyword type: Real
///
/// Default:
/// If SPIN is specified: NUP = (NELECTRONS + SPIN) / 2
/// Else: NUP = NELECTRONS / 2
///
/// Example:
/// NUP : 12.0
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "NUP")]
pub struct Nup(pub f64);

// Note: Default logic is context-dependent.
// The `Default` implementation here is omitted as it's not directly applicable.
// A containing struct or the application logic would need to handle this.

impl ToCell for Nup {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("NUP", CellValue::Float(self.0))
    }
}

impl ToCellValue for Nup {
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
    fn test_nup_serde() {
        let nup_str = "NUP : 12.0";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithNup {
            nup: Nup,
        }

        let cell_file_result: Result<CellFileWithNup, _> = from_str(nup_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.nup.0 - 12.0).abs() < f64::EPSILON);

        let nup_instance = Nup(8.5);
        let serialized_result = to_string(&nup_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized NUP (8.5): {serialized_string}");
        assert!(serialized_string.contains("NUP"));
        assert!(serialized_string.contains("8.5"));
    }
}
