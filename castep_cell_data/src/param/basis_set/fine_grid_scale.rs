// File: basis_set/fine_grid_scale.rs
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the maximum size of the g-vectors included in the fine grid
/// relative to the standard grid.
///
/// Keyword type: Real
///
/// Default: 1.0 (results in fine and standard grids being identical)
///
/// Example:
/// FINE_GRID_SCALE : 2.0
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "FINE_GRID_SCALE")]
pub struct FineGridScale(pub f64);

impl Default for FineGridScale {
    fn default() -> Self {
        Self(1.0) // Default is 1.0
    }
}

impl ToCell for FineGridScale {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("FINE_GRID_SCALE", CellValue::Float(self.0))
    }
}

impl ToCellValue for FineGridScale {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Float(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_fine_grid_scale_serde() {
        let fine_grid_scale_str = "FINE_GRID_SCALE : 2.0";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithFineGridScale {
            fine_grid_scale: FineGridScale,
        }

        let cell_file_result: Result<CellFileWithFineGridScale, _> = from_str(fine_grid_scale_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.fine_grid_scale.0 - 2.0).abs() < f64::EPSILON);

        let fine_grid_scale_instance = FineGridScale(1.5);
        let serialized_result = to_string(&fine_grid_scale_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized FINE_GRID_SCALE (1.5): {serialized_string}");
        assert!(serialized_string.contains("FINE_GRID_SCALE"));
        assert!(serialized_string.contains("1.5"));

        assert_eq!(FineGridScale::default(), FineGridScale(1.0));
    }
}
