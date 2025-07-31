use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the maximum number of bands at any k-point and spin.
///
/// Keyword type: Integer
///
/// Default: Calculated based on NELECTRONS/NUP+NDOWN and NEXTRA_BANDS/PERC_EXTRA_BANDS
///
/// Example:
/// NBANDS : 64
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "NBANDS")]
pub struct Nbands(pub u32); // Using i32 for potential negative checks/validation if needed

// Note: Default logic is complex and context-dependent.
// The `Default` implementation here is omitted as it's not directly applicable.
// A containing struct or the application logic would need to handle this.

impl ToCell for Nbands {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("NBANDS", CellValue::UInt(self.0))
    }
}

impl ToCellValue for Nbands {
    fn to_cell_value(&self) -> CellValue {
        CellValue::UInt(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_nbands_serde() {
        let nbands_str = "NBANDS : 64";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithNbands {
            nbands: Nbands,
        }

        let cell_file_result: Result<CellFileWithNbands, _> = from_str(nbands_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.nbands.0, 64);

        let nbands_instance = Nbands(128);
        let serialized_result = to_string(&nbands_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized NBANDS (128): {serialized_string}");
        assert!(serialized_string.contains("NBANDS"));
        assert!(serialized_string.contains("128"));
    }
}
