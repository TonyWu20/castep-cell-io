use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the maximum number of SCF cycles performed in an electronic minimization.
///
/// Keyword type: Integer
///
/// Default: 30
///
/// Example:
/// MAX_SCF_CYCLES : 20
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MAX_SCF_CYCLES")]
pub struct MaxScfCycles(pub i32);

impl Default for MaxScfCycles {
    fn default() -> Self {
        Self(30) // Default is 30
    }
}

impl ToCell for MaxScfCycles {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MAX_SCF_CYCLES", CellValue::Int(self.0))
    }
}

impl ToCellValue for MaxScfCycles {
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
    fn test_max_scf_cycles_serde() {
        let max_scf_cycles_str = "MAX_SCF_CYCLES : 20";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMaxScfCycles {
            max_scf_cycles: MaxScfCycles,
        }

        let cell_file_result: Result<CellFileWithMaxScfCycles, _> = from_str(max_scf_cycles_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.max_scf_cycles.0, 20);

        let max_scf_cycles_instance = MaxScfCycles(50);
        let serialized_result = to_string(&max_scf_cycles_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MAX_SCF_CYCLES (50):\n{serialized_string}");
        assert!(serialized_string.contains("MAX_SCF_CYCLES"));
        assert!(serialized_string.contains("50"));

        assert_eq!(MaxScfCycles::default(), MaxScfCycles(30));
    }
}
