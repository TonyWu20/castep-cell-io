// File: geometry_optimization/geom_convergence_win.rs
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the size of the convergence window for a geometry optimization.
///
/// Keyword type: Integer
///
/// Default: 2
///
/// Example:
/// GEOM_CONVERGENCE_WIN : 4
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "GEOM_CONVERGENCE_WIN")]
pub struct GeomConvergenceWin(pub i32); // Using i32

impl Default for GeomConvergenceWin {
    fn default() -> Self {
        Self(2) // Default is 2
    }
}

impl ToCell for GeomConvergenceWin {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("GEOM_CONVERGENCE_WIN", CellValue::Int(self.0))
    }
}

impl ToCellValue for GeomConvergenceWin {
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
    fn test_geom_convergence_win_serde() {
        let geom_convergence_win_str = "GEOM_CONVERGENCE_WIN : 4";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithGeomConvergenceWin {
            geom_convergence_win: GeomConvergenceWin,
        }

        let cell_file_result: Result<CellFileWithGeomConvergenceWin, _> =
            from_str(geom_convergence_win_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.geom_convergence_win.0, 4);

        let geom_convergence_win_instance = GeomConvergenceWin(5);
        let serialized_result = to_string(&geom_convergence_win_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized GEOM_CONVERGENCE_WIN (5): {serialized_string}");
        assert!(serialized_string.contains("GEOM_CONVERGENCE_WIN"));
        assert!(serialized_string.contains("5"));

        assert_eq!(GeomConvergenceWin::default(), GeomConvergenceWin(2));
    }
}
