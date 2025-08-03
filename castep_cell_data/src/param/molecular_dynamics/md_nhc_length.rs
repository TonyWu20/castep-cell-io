use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the length of Nosé-Hoover thermostat chains.
///
/// Keyword type: Integer
///
/// Default: 5
///
/// Example:
/// MD_NHC_LENGTH : 3
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MD_NHC_LENGTH")]
pub struct MdNhcLength(pub u32); // Using u32 as it's a count/length

impl Default for MdNhcLength {
    fn default() -> Self {
        Self(5) // Default is 5
    }
}

impl ToCell for MdNhcLength {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_NHC_LENGTH", CellValue::UInt(self.0))
    }
}

impl ToCellValue for MdNhcLength {
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
    fn test_md_nhc_length_serde() {
        let md_nhc_length_str = "MD_NHC_LENGTH : 3";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMdNhcLength {
            md_nhc_length: MdNhcLength,
        }

        let cell_file_result: Result<CellFileWithMdNhcLength, _> = from_str(md_nhc_length_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.md_nhc_length.0, 3);

        let md_nhc_length_instance = MdNhcLength(7);
        let serialized_result = to_string(&md_nhc_length_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MD_NHC_LENGTH (7): {serialized_string}");
        assert!(serialized_string.contains("MD_NHC_LENGTH"));
        assert!(serialized_string.contains("7"));

        assert_eq!(MdNhcLength::default(), MdNhcLength(5));
    }
}
