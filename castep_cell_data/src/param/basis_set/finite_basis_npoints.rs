use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the number of points used to estimate the BASIS_DE_DLOGE
/// in automatic calculation of the finite basis set correction.
///
/// Keyword type: Integer
///
/// Default: 3
///
/// Example:
/// FINITE_BASIS_NPOINTS : 5
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "FINITE_BASIS_NPOINTS")]
pub struct FiniteBasisNpoints(pub i32); // Using i32 to allow for potential negative checks if needed

impl Default for FiniteBasisNpoints {
    fn default() -> Self {
        Self(3) // Default is 3
    }
}

impl ToCell for FiniteBasisNpoints {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("FINITE_BASIS_NPOINTS", CellValue::Int(self.0))
    }
}

impl ToCellValue for FiniteBasisNpoints {
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
    fn test_finite_basis_npoints_serde() {
        let finite_basis_npoints_str = "FINITE_BASIS_NPOINTS : 5";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithFiniteBasisNpoints {
            finite_basis_npoints: FiniteBasisNpoints,
        }

        let cell_file_result: Result<CellFileWithFiniteBasisNpoints, _> =
            from_str(finite_basis_npoints_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.finite_basis_npoints.0, 5);

        let finite_basis_npoints_instance = FiniteBasisNpoints(10);
        let serialized_result = to_string(&finite_basis_npoints_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized FINITE_BASIS_NPOINTS (10): {serialized_string}");
        assert!(serialized_string.contains("FINITE_BASIS_NPOINTS"));
        assert!(serialized_string.contains("10"));

        assert_eq!(FiniteBasisNpoints::default(), FiniteBasisNpoints(3));
    }
}
