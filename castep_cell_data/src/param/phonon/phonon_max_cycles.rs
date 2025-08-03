use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the maximum number of iterations in a phonon calculation.
///
/// Keyword type: Integer
///
/// Default: 50
///
/// Example:
/// PHONON_MAX_CYCLES : 30
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "PHONON_MAX_CYCLES")]
pub struct PhononMaxCycles(pub u32); // Using i32

impl Default for PhononMaxCycles {
    fn default() -> Self {
        Self(50) // Default is 50
    }
}

impl ToCell for PhononMaxCycles {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PHONON_MAX_CYCLES", CellValue::UInt(self.0))
    }
}

impl ToCellValue for PhononMaxCycles {
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
    fn test_phonon_max_cycles_serde() {
        let phonon_max_cycles_str = "PHONON_MAX_CYCLES : 30";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPhononMaxCycles {
            phonon_max_cycles: PhononMaxCycles,
        }

        let cell_file_result: Result<CellFileWithPhononMaxCycles, _> =
            from_str(phonon_max_cycles_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.phonon_max_cycles.0, 30);

        let phonon_max_cycles_instance = PhononMaxCycles(100);
        let serialized_result = to_string(&phonon_max_cycles_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized PHONON_MAX_CYCLES (100): {serialized_string}");
        assert!(serialized_string.contains("PHONON_MAX_CYCLES"));
        assert!(serialized_string.contains("100"));

        assert_eq!(PhononMaxCycles::default(), PhononMaxCycles(50));
    }
}
