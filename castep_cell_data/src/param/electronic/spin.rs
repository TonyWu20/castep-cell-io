use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the initial value for the number of unpaired electrons
/// in a spin-polarized calculation.
///
/// Keyword type: Real
///
/// Default:
/// 0.0 when the total number of electrons is even.
/// 1.0 when the total number of electrons is odd.
///
/// Example:
/// SPIN : 3.0
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "SPIN")]
pub struct Spin(pub f64);

// Note: Default logic is context-dependent (depends on NELECTRONS).
// The `Default` implementation here is omitted as it's not directly applicable.
// A containing struct or the application logic would need to handle this.
// If a simple default is needed, it could be implemented, but the spec's logic is more complex.

impl ToCell for Spin {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("SPIN", CellValue::Float(self.0))
    }
}

impl ToCellValue for Spin {
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
    fn test_spin_serde() {
        let spin_str = "SPIN : 3.0";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithSpin {
            spin: Spin,
        }

        let cell_file_result: Result<CellFileWithSpin, _> = from_str(spin_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.spin.0 - 3.0).abs() < f64::EPSILON);

        let spin_instance = Spin(1.0);
        let serialized_result = to_string(&spin_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized SPIN (1.0): {serialized_string}");
        assert!(serialized_string.contains("SPIN"));
        assert!(serialized_string.contains("1.0"));
    }
}
