use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the initialization of random number seeds.
///
/// Keyword type: Integer (expert)
///
/// Default: 0
///
/// Example:
/// RAND_SEED : -25
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "RAND_SEED")]
pub struct RandSeed(pub i32);

impl ToCell for RandSeed {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("RAND_SEED", CellValue::Int(self.0))
    }
}

impl ToCellValue for RandSeed {
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
    fn test_rand_seed_serde() {
        let rand_seed_str = "RAND_SEED : -25";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithRandSeed {
            rand_seed: RandSeed,
        }

        let cell_file_result: Result<CellFileWithRandSeed, _> = from_str(rand_seed_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.rand_seed.0, -25);

        let rand_seed_instance = RandSeed(42);
        let serialized_result = to_string(&rand_seed_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized RAND_SEED (42):\n{serialized_string}");
        assert!(serialized_string.contains("RAND_SEED"));
        assert!(serialized_string.contains("42"));

        assert_eq!(RandSeed::default(), RandSeed(0));
    }
}
