// File: basis_set/fixed_npw.rs
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines whether a fixed number of plane waves (TRUE) or a fixed
/// plane wave cutoff energy (FALSE) will be used when doing a variable cell calculation.
///
/// Keyword type: Logical
///
/// Default: false
///
/// Example:
/// FIXED_NPW : TRUE
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "FIXED_NPW")]
pub struct FixedNpw(pub bool);

impl ToCell for FixedNpw {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("FIXED_NPW", CellValue::Bool(self.0))
    }
}

impl ToCellValue for FixedNpw {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_fixed_npw_serde() {
        let fixed_npw_true_str = "FIXED_NPW : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithFixedNpwTrue {
            fixed_npw: FixedNpw,
        }

        let cell_file_true_result: Result<CellFileWithFixedNpwTrue, _> =
            from_str(fixed_npw_true_str);
        assert!(
            cell_file_true_result.is_ok(),
            "Deserialization (TRUE) failed: {:?}",
            cell_file_true_result.err()
        );
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.fixed_npw.0);

        let fixed_npw_false_str = "FIXED_NPW : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithFixedNpwFalse {
            fixed_npw: FixedNpw,
        }

        let cell_file_false_result: Result<CellFileWithFixedNpwFalse, _> =
            from_str(fixed_npw_false_str);
        assert!(
            cell_file_false_result.is_ok(),
            "Deserialization (FALSE) failed: {:?}",
            cell_file_false_result.err()
        );
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.fixed_npw.0);

        let fixed_npw_instance = FixedNpw(true);
        let serialized_result = to_string(&fixed_npw_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized FIXED_NPW (TRUE): {serialized_string}");
        assert!(serialized_string.contains("FIXED_NPW"));
        assert!(serialized_string.contains("true") || serialized_string.contains("TRUE"));

        assert_eq!(FixedNpw::default(), FixedNpw(false));
    }
}
