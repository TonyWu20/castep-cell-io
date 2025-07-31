use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the total number of electrons in the system.
///
/// Keyword type: Real
///
/// Default:
/// If CHARGE is specified, NELECTRONS is chosen to match the charge.
/// If NUP and NDOWN are specified, NELECTRONS = NUP + NDOWN.
/// Otherwise, a default value for a neutral system is chosen.
///
/// Example:
/// NELECTRONS : 3.0
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "NELECTRONS")]
pub struct Nelectrons(pub f64);

// Note: Default logic is complex and context-dependent.
// The `Default` implementation here is omitted as it's not directly applicable.
// A containing struct or the application logic would need to handle this.

impl ToCell for Nelectrons {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("NELECTRONS", CellValue::Float(self.0))
    }
}

impl ToCellValue for Nelectrons {
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
    fn test_nelectrons_serde() {
        let nelectrons_str = "NELECTRONS : 3.0";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithNelectrons {
            nelectrons: Nelectrons,
        }

        let cell_file_result: Result<CellFileWithNelectrons, _> = from_str(nelectrons_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.nelectrons.0 - 3.0).abs() < f64::EPSILON);

        let nelectrons_instance = Nelectrons(10.0);
        let serialized_result = to_string(&nelectrons_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized NELECTRONS (10.0): {serialized_string}");
        assert!(serialized_string.contains("NELECTRONS"));
        assert!(serialized_string.contains("10.0"));
    }
}
