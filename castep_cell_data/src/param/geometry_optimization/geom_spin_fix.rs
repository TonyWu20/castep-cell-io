use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the number of geometry optimization steps for which the total spin is fixed.
///
/// Keyword type: Integer
///
/// Default: 0 (spin is allowed to vary)
///
/// Example:
/// GEOM_SPIN_FIX : 5
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "GEOM_SPIN_FIX")]
pub struct GeomSpinFix(pub i32); // Using i32 to allow negative values

impl ToCell for GeomSpinFix {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("GEOM_SPIN_FIX", CellValue::Int(self.0))
    }
}

impl ToCellValue for GeomSpinFix {
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
    fn test_geom_spin_fix_serde() {
        let geom_spin_fix_str = "GEOM_SPIN_FIX : 5";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithGeomSpinFix {
            geom_spin_fix: GeomSpinFix,
        }

        let cell_file_result: Result<CellFileWithGeomSpinFix, _> = from_str(geom_spin_fix_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.geom_spin_fix.0, 5);

        let geom_spin_fix_instance = GeomSpinFix(-1);
        let serialized_result = to_string(&geom_spin_fix_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized GEOM_SPIN_FIX (-1): {serialized_string}");
        assert!(serialized_string.contains("GEOM_SPIN_FIX"));
        assert!(serialized_string.contains("-1"));

        assert_eq!(GeomSpinFix::default(), GeomSpinFix(0));
    }
}
