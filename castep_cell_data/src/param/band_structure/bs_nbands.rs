use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the number of bands at each k-point when performing a band structure calculation.
///
/// Keyword type: Integer
///
/// Default: NBANDS + 5√NBANDS (context-dependent, not directly representable here)
///
/// Example:
/// BS_NBANDS : 64
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "BS_NBANDS")]
pub struct BsNbands(pub u32); // Using i32

// Note: Default logic is complex and context-dependent.
// The `Default` implementation here is omitted as it's not directly applicable.
// A containing struct or the application logic would need to handle this.

impl ToCell for BsNbands {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("BS_NBANDS", CellValue::UInt(self.0))
    }
}

impl ToCellValue for BsNbands {
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
    fn test_bs_nbands_serde() {
        let bs_nbands_str = "BS_NBANDS : 64";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithBsNbands {
            bs_nbands: BsNbands,
        }

        let cell_file_result: Result<CellFileWithBsNbands, _> = from_str(bs_nbands_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.bs_nbands.0, 64);

        let bs_nbands_instance = BsNbands(128);
        let serialized_result = to_string(&bs_nbands_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized BS_NBANDS (128): {serialized_string}");
        assert!(serialized_string.contains("BS_NBANDS"));
        assert!(serialized_string.contains("128"));
    }
}
