use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the number of extra bands in addition to the number of occupied bands.
///
/// Keyword type: Integer
///
/// Default: 0
///
/// Example:
/// NEXTRA_BANDS : 12
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "NEXTRA_BANDS")]
pub struct NextraBands(pub u32);

impl ToCell for NextraBands {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("NEXTRA_BANDS", CellValue::UInt(self.0))
    }
}

impl ToCellValue for NextraBands {
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
    fn test_nextra_bands_serde() {
        let nextra_bands_str = "NEXTRA_BANDS : 12";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithNextraBands {
            nextra_bands: NextraBands,
        }

        let cell_file_result: Result<CellFileWithNextraBands, _> = from_str(nextra_bands_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.nextra_bands.0, 12);

        let nextra_bands_instance = NextraBands(20);
        let serialized_result = to_string(&nextra_bands_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized NEXTRA_BANDS (20): {serialized_string}");
        assert!(serialized_string.contains("NEXTRA_BANDS"));
        assert!(serialized_string.contains("20"));

        assert_eq!(NextraBands::default(), NextraBands(0));
    }
}
