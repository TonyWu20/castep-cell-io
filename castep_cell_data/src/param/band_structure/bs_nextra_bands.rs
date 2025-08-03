use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the number of extra bands at each k-point in addition to the number of occupied bands.
///
/// Keyword type: Integer
///
/// Default: 0
///
/// Example:
/// BS_NEXTRA_BANDS : 12
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "BS_NEXTRA_BANDS")]
pub struct BsNextraBands(pub u32); // Using i32

impl ToCell for BsNextraBands {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("BS_NEXTRA_BANDS", CellValue::UInt(self.0))
    }
}

impl ToCellValue for BsNextraBands {
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
    fn test_bs_nextra_bands_serde() {
        let bs_nextra_bands_str = "BS_NEXTRA_BANDS : 12";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithBsNextraBands {
            bs_nextra_bands: BsNextraBands,
        }

        let cell_file_result: Result<CellFileWithBsNextraBands, _> = from_str(bs_nextra_bands_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.bs_nextra_bands.0, 12);

        let bs_nextra_bands_instance = BsNextraBands(20);
        let serialized_result = to_string(&bs_nextra_bands_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized BS_NEXTRA_BANDS (20): {serialized_string}");
        assert!(serialized_string.contains("BS_NEXTRA_BANDS"));
        assert!(serialized_string.contains("20"));

        assert_eq!(BsNextraBands::default(), BsNextraBands(0));
    }
}
