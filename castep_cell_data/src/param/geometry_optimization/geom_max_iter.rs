use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the maximum number of steps in a geometry optimization.
///
/// Keyword type: Integer
///
/// Default: 30
///
/// Example:
/// GEOM_MAX_ITER : 25
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "GEOM_MAX_ITER")]
pub struct GeomMaxIter(pub u32);

impl Default for GeomMaxIter {
    fn default() -> Self {
        Self(30) // Default is 30
    }
}

impl ToCell for GeomMaxIter {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("GEOM_MAX_ITER", CellValue::UInt(self.0))
    }
}

impl ToCellValue for GeomMaxIter {
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
    fn test_geom_max_iter_serde() {
        let geom_max_iter_str = "GEOM_MAX_ITER : 25";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithGeomMaxIter {
            geom_max_iter: GeomMaxIter,
        }

        let cell_file_result: Result<CellFileWithGeomMaxIter, _> = from_str(geom_max_iter_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.geom_max_iter.0, 25);

        let geom_max_iter_instance = GeomMaxIter(50);
        let serialized_result = to_string(&geom_max_iter_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized GEOM_MAX_ITER (50): {serialized_string}");
        assert!(serialized_string.contains("GEOM_MAX_ITER"));
        assert!(serialized_string.contains("50"));

        assert_eq!(GeomMaxIter::default(), GeomMaxIter(30));
    }
}
