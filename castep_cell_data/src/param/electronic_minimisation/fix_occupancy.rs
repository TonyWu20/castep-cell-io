use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies whether or not the occupancies of the bands should be fixed.
///
/// Keyword type: Logical
///
/// Default: false
///
/// Example:
/// FIX_OCCUPANCY : TRUE
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "FIX_OCCUPANCY")]
pub struct FixOccupancy(pub bool);

impl ToCell for FixOccupancy {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("FIX_OCCUPANCY", CellValue::Bool(self.0))
    }
}

impl ToCellValue for FixOccupancy {
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
    fn test_fix_occupancy_serde() {
        let fix_occupancy_true_str = "FIX_OCCUPANCY : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithFixOccupancyTrue {
            fix_occupancy: FixOccupancy,
        }

        let cell_file_true_result: Result<CellFileWithFixOccupancyTrue, _> =
            from_str(fix_occupancy_true_str);
        assert!(
            cell_file_true_result.is_ok(),
            "Deserialization (TRUE) failed: {:?}",
            cell_file_true_result.err()
        );
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.fix_occupancy.0);

        let fix_occupancy_false_str = "FIX_OCCUPANCY : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithFixOccupancyFalse {
            fix_occupancy: FixOccupancy,
        }

        let cell_file_false_result: Result<CellFileWithFixOccupancyFalse, _> =
            from_str(fix_occupancy_false_str);
        assert!(
            cell_file_false_result.is_ok(),
            "Deserialization (FALSE) failed: {:?}",
            cell_file_false_result.err()
        );
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.fix_occupancy.0);

        let fix_occupancy_instance = FixOccupancy(true);
        let serialized_result = to_string(&fix_occupancy_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized FIX_OCCUPANCY (TRUE): {serialized_string}");
        assert!(serialized_string.contains("FIX_OCCUPANCY"));
        assert!(serialized_string.contains("true") || serialized_string.contains("TRUE"));

        assert_eq!(FixOccupancy::default(), FixOccupancy(false));
    }
}
