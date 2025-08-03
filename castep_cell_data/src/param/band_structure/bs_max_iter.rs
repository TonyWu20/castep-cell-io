use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the maximum number of iterations to perform when calculating band structure.
///
/// Keyword type: Integer
///
/// Default: 60
///
/// Example:
/// BS_MAX_ITER : 50
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "BS_MAX_ITER")]
pub struct BsMaxIter(pub u32); // Using i32

impl Default for BsMaxIter {
    fn default() -> Self {
        Self(60) // Default is 60
    }
}

impl ToCell for BsMaxIter {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("BS_MAX_ITER", CellValue::UInt(self.0))
    }
}

impl ToCellValue for BsMaxIter {
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
    fn test_bs_max_iter_serde() {
        let bs_max_iter_str = "BS_MAX_ITER : 50";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithBsMaxIter {
            bs_max_iter: BsMaxIter,
        }

        let cell_file_result: Result<CellFileWithBsMaxIter, _> = from_str(bs_max_iter_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.bs_max_iter.0, 50);

        let bs_max_iter_instance = BsMaxIter(100);
        let serialized_result = to_string(&bs_max_iter_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized BS_MAX_ITER (100): {serialized_string}");
        assert!(serialized_string.contains("BS_MAX_ITER"));
        assert!(serialized_string.contains("100"));

        assert_eq!(BsMaxIter::default(), BsMaxIter(60));
    }
}
