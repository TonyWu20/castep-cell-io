use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};
// Assuming EnergyUnit exists in units module
use crate::units::EnergyUnit;

/// Controls the tolerance for accepting convergence of a single eigenvalue or band.
///
/// Keyword type: Real
///
/// Default: 1e-6 eV
///
/// Example:
/// BS_EIGENVALUE_TOL = 1.0e-5 Ha
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "BS_EIGENVALUE_TOL")]
#[serde(from = "BsEigenvalueTolRepr")] // Use intermediate repr for deserialization
pub struct BsEigenvalueTol {
    /// The eigenvalue tolerance value.
    pub value: f64,
    /// The optional unit of the energy value.
    pub unit: Option<EnergyUnit>,
}

/// Intermediate representation for deserializing `BsEigenvalueTol`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum BsEigenvalueTolRepr {
    /// Format: value unit
    WithUnit(f64, EnergyUnit),
    /// Format: value (default unit eV implied)
    Essential(f64),
}

impl From<BsEigenvalueTolRepr> for BsEigenvalueTol {
    fn from(repr: BsEigenvalueTolRepr) -> Self {
        match repr {
            BsEigenvalueTolRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            BsEigenvalueTolRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit (eV) implied
            },
        }
    }
}

impl ToCell for BsEigenvalueTol {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("BS_EIGENVALUE_TOL", self.to_cell_value())
    }
}

impl ToCellValue for BsEigenvalueTol {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(
            [
                CellValue::Float(self.value),
                self.unit
                    .as_ref()
                    .map(|u| u.to_cell_value())
                    .unwrap_or(CellValue::Null),
            ]
            .to_vec(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_bs_eigenvalue_tol_serde() {
        // 1. Test Deserialization with unit
        let bs_eigenvalue_tol_with_unit_str = "BS_EIGENVALUE_TOL : 1.0e-5 ha";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithBsEigenvalueTolUnit {
            bs_eigenvalue_tol: BsEigenvalueTol,
        }

        let cell_file_result: Result<CellFileWithBsEigenvalueTolUnit, _> =
            from_str(bs_eigenvalue_tol_with_unit_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.bs_eigenvalue_tol.value - 1.0e-5).abs() < 1e-15); // Adjust epsilon for scientific notation
        assert_eq!(cell_file.bs_eigenvalue_tol.unit, Some(EnergyUnit::Hartree));

        // 2. Test Deserialization without unit (default unit implied)
        let bs_eigenvalue_tol_default_str = "BS_EIGENVALUE_TOL : 1e-6";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithBsEigenvalueTolDefault {
            bs_eigenvalue_tol: BsEigenvalueTol,
        }

        let cell_file_default_result: Result<CellFileWithBsEigenvalueTolDefault, _> =
            from_str(bs_eigenvalue_tol_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.bs_eigenvalue_tol.value - 1e-6).abs() < f64::EPSILON);
        assert_eq!(cell_file_default.bs_eigenvalue_tol.unit, None);

        // 3. Test Serialization using ToCell (with unit)
        let bs_eigenvalue_tol_instance_with_unit = BsEigenvalueTol {
            value: 5e-7,
            unit: Some(EnergyUnit::ElectronVolt),
        };
        let serialized_result_with_unit =
            to_string(&bs_eigenvalue_tol_instance_with_unit.to_cell());
        assert!(
            serialized_result_with_unit.is_ok(),
            "Serialization (with unit) failed: {:?}",
            serialized_result_with_unit.err()
        );
        let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        println!("Serialized BS_EIGENVALUE_TOL (5e-7 ev): {serialized_string_with_unit}");
        assert!(serialized_string_with_unit.contains("BS_EIGENVALUE_TOL"));
        assert!(
            serialized_string_with_unit.contains("5e-7")
                || serialized_string_with_unit.contains("0.0000005")
        );
        assert!(serialized_string_with_unit.contains("ev"));

        // 4. Test Serialization using ToCell (without unit)
        let bs_eigenvalue_tol_instance_no_unit = BsEigenvalueTol {
            value: 2e-6,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&bs_eigenvalue_tol_instance_no_unit.to_cell());
        assert!(
            serialized_result_no_unit.is_ok(),
            "Serialization (no unit) failed: {:?}",
            serialized_result_no_unit.err()
        );
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized BS_EIGENVALUE_TOL (2e-6, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("BS_EIGENVALUE_TOL"));
        assert!(
            serialized_string_no_unit.contains("2e-6")
                || serialized_string_no_unit.contains("0.000002")
        );
        // Check that the unit string is not present (or is the default)
    }
}
