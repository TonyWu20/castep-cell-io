use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};
// Assuming EnergyUnit exists in units module
use crate::units::EnergyUnit;

/// Controls the tolerance for accepting convergence of a single eigenvalue during density mixing minimization.
///
/// Keyword type: Real
///
/// Default: The lower of 1e-6 eV and ELEC_ENERGY_TOL*NATOMS/NBANDS
///
/// Example:
/// ELEC_EIGENVALUE_TOL : 0.000007 eV
/// ELEC_EIGENVALUE_TOL : 0.000007 (uses default unit, likely eV)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "ELEC_EIGENVALUE_TOL")]
#[serde(from = "ElecEigenvalueTolRepr")] // Use intermediate repr for deserialization
pub struct ElecEigenvalueTol {
    /// The eigenvalue tolerance value.
    pub value: f64,
    /// The optional unit of the energy value.
    /// If None, a default unit (likely eV) is implied.
    pub unit: Option<EnergyUnit>,
}

/// Intermediate representation for deserializing `ElecEigenvalueTol`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ElecEigenvalueTolRepr {
    /// Format: value unit
    WithUnit(f64, EnergyUnit),
    /// Format: value (default unit implied)
    Essential(f64),
}

impl From<ElecEigenvalueTolRepr> for ElecEigenvalueTol {
    fn from(repr: ElecEigenvalueTolRepr) -> Self {
        match repr {
            ElecEigenvalueTolRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            ElecEigenvalueTolRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit implied
            },
        }
    }
}

impl ToCell for ElecEigenvalueTol {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("ELEC_EIGENVALUE_TOL", self.to_cell_value())
    }
}

impl ToCellValue for ElecEigenvalueTol {
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
    fn test_elec_eigenvalue_tol_serde() {
        // 1. Test Deserialization with unit
        let elec_eigenvalue_tol_with_unit_str = "ELEC_EIGENVALUE_TOL : 0.000007 ev";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithElecEigenvalueTolUnit {
            elec_eigenvalue_tol: ElecEigenvalueTol,
        }

        let cell_file_result: Result<CellFileWithElecEigenvalueTolUnit, _> =
            from_str(elec_eigenvalue_tol_with_unit_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.elec_eigenvalue_tol.value - 0.000007).abs() < 1e-10);
        assert_eq!(
            cell_file.elec_eigenvalue_tol.unit,
            Some(EnergyUnit::ElectronVolt)
        );

        // 2. Test Deserialization without unit (default unit implied)
        let elec_eigenvalue_tol_default_str = "ELEC_EIGENVALUE_TOL : 0.000001";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithElecEigenvalueTolDefault {
            elec_eigenvalue_tol: ElecEigenvalueTol,
        }

        let cell_file_default_result: Result<CellFileWithElecEigenvalueTolDefault, _> =
            from_str(elec_eigenvalue_tol_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.elec_eigenvalue_tol.value - 0.000001).abs() < 1e-10);
        assert_eq!(cell_file_default.elec_eigenvalue_tol.unit, None);

        // 3. Test Serialization using ToCell (with unit)
        let elec_eigenvalue_tol_instance_with_unit = ElecEigenvalueTol {
            value: 1e-6,
            unit: Some(EnergyUnit::Hartree),
        };
        let serialized_result_with_unit =
            to_string(&elec_eigenvalue_tol_instance_with_unit.to_cell());
        assert!(
            serialized_result_with_unit.is_ok(),
            "Serialization (with unit) failed: {:?}",
            serialized_result_with_unit.err()
        );
        let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        println!("Serialized ELEC_EIGENVALUE_TOL (1e-6 ha): {serialized_string_with_unit}");
        assert!(serialized_string_with_unit.contains("ELEC_EIGENVALUE_TOL"));
        assert!(
            serialized_string_with_unit.contains("1e-6")
                || serialized_string_with_unit.contains("0.000001")
        );
        assert!(serialized_string_with_unit.contains("ha"));

        // 4. Test Serialization using ToCell (without unit)
        let elec_eigenvalue_tol_instance_no_unit = ElecEigenvalueTol {
            value: 2e-6,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&elec_eigenvalue_tol_instance_no_unit.to_cell());
        assert!(
            serialized_result_no_unit.is_ok(),
            "Serialization (no unit) failed: {:?}",
            serialized_result_no_unit.err()
        );
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized ELEC_EIGENVALUE_TOL (2e-6, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("ELEC_EIGENVALUE_TOL"));
        assert!(
            serialized_string_no_unit.contains("2e-6")
                || serialized_string_no_unit.contains("0.000002")
        );
        // Check that the unit string is not present
        assert!(!serialized_string_no_unit.contains("ev"));
        assert!(!serialized_string_no_unit.contains("ha"));
        // ... (add checks for other unit strings if necessary)
    }
}
