use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the maximum number of iterations in a linear response to electric field calculation.
///
/// Keyword type: Integer
///
/// Default: 50
///
/// Example:
/// EFIELD_MAX_CYCLES : 100
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "EFIELD_MAX_CYCLES")]
pub struct EfieldMaxCycles(pub u32); // Using u32 as it's a count of cycles

impl Default for EfieldMaxCycles {
    fn default() -> Self {
        Self(50) // Default is 50
    }
}

impl ToCell for EfieldMaxCycles {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("EFIELD_MAX_CYCLES", CellValue::UInt(self.0))
    }
}

impl ToCellValue for EfieldMaxCycles {
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
    fn test_efield_max_cycles_serde() {
        let efield_max_cycles_str = "EFIELD_MAX_CYCLES : 100";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithEfieldMaxCycles {
            efield_max_cycles: EfieldMaxCycles,
        }

        let cell_file_result: Result<CellFileWithEfieldMaxCycles, _> =
            from_str(efield_max_cycles_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.efield_max_cycles.0, 100);

        let efield_max_cycles_instance = EfieldMaxCycles(200);
        let serialized_result = to_string(&efield_max_cycles_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized EFIELD_MAX_CYCLES (200): {serialized_string}");
        assert!(serialized_string.contains("EFIELD_MAX_CYCLES"));
        assert!(serialized_string.contains("200"));

        assert_eq!(EfieldMaxCycles::default(), EfieldMaxCycles(50));
    }
}
