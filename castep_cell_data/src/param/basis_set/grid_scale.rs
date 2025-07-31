use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the size of the standard grid, relative to the diameter
/// of the cutoff sphere.
///
/// Keyword type: Real
///
/// Default: 1.75
///
/// Example:
/// GRID_SCALE : 2.0
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "GRID_SCALE")]
pub struct GridScale(pub f64);

impl Default for GridScale {
    fn default() -> Self {
        Self(1.75) // Default is 1.75
    }
}

impl ToCell for GridScale {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("GRID_SCALE", CellValue::Float(self.0))
    }
}

impl ToCellValue for GridScale {
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
    fn test_grid_scale_serde() {
        let grid_scale_str = "GRID_SCALE : 2.0";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithGridScale {
            grid_scale: GridScale,
        }

        let cell_file_result: Result<CellFileWithGridScale, _> = from_str(grid_scale_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.grid_scale.0 - 2.0).abs() < f64::EPSILON);

        let grid_scale_instance = GridScale(1.5);
        let serialized_result = to_string(&grid_scale_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized GRID_SCALE (1.5): {serialized_string}");
        assert!(serialized_string.contains("GRID_SCALE"));
        assert!(serialized_string.contains("1.5"));

        assert_eq!(GridScale::default(), GridScale(1.75));
    }
}
