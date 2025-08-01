use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the number of SCF cycles between updates to the wavefunction and density data file.
///
/// Keyword type: Integer
///
/// Default: 5
///
/// Example:
/// NUM_DUMP_CYCLES : 10
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "NUM_DUMP_CYCLES")]
pub struct NumDumpCycles(pub i32); // Using i32 to allow for <= 0 values

impl Default for NumDumpCycles {
    fn default() -> Self {
        Self(5) // Default is 5
    }
}

impl ToCell for NumDumpCycles {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("NUM_DUMP_CYCLES", CellValue::Int(self.0))
    }
}

impl ToCellValue for NumDumpCycles {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_num_dump_cycles_serde() {
        let num_dump_cycles_str = "NUM_DUMP_CYCLES : 10";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithNumDumpCycles {
            num_dump_cycles: NumDumpCycles,
        }

        let cell_file_result: Result<CellFileWithNumDumpCycles, _> = from_str(num_dump_cycles_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.num_dump_cycles.0, 10);

        let num_dump_cycles_instance = NumDumpCycles(0);
        let serialized_result = to_string(&num_dump_cycles_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized NUM_DUMP_CYCLES (0): {serialized_string}");
        assert!(serialized_string.contains("NUM_DUMP_CYCLES"));
        assert!(serialized_string.contains("0"));

        assert_eq!(NumDumpCycles::default(), NumDumpCycles(5));
    }
}
