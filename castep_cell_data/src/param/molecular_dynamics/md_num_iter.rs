use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the total number of steps for a molecular dynamics calculation.
///
/// Keyword type: Integer
///
/// Default: 100
///
/// Example:
/// MD_NUM_ITER : 125
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MD_NUM_ITER")]
pub struct MdNumIter(pub u32); // Using u32 as it's a count of iterations

impl Default for MdNumIter {
    fn default() -> Self {
        Self(100) // Default is 100
    }
}

impl ToCell for MdNumIter {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_NUM_ITER", CellValue::UInt(self.0))
    }
}

impl ToCellValue for MdNumIter {
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
    fn test_md_num_iter_serde() {
        let md_num_iter_str = "MD_NUM_ITER : 125";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMdNumIter {
            md_num_iter: MdNumIter,
        }

        let cell_file_result: Result<CellFileWithMdNumIter, _> = from_str(md_num_iter_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.md_num_iter.0, 125);

        let md_num_iter_instance = MdNumIter(250);
        let serialized_result = to_string(&md_num_iter_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MD_NUM_ITER (250): {serialized_string}");
        assert!(serialized_string.contains("MD_NUM_ITER"));
        assert!(serialized_string.contains("250"));

        assert_eq!(MdNumIter::default(), MdNumIter(100));
    }
}
