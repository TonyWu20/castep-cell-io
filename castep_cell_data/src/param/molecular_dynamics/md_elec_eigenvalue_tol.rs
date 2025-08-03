use crate::units::EnergyUnit;
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the tolerance for accepting convergence of a single eigenvalue during MD.
///
/// Keyword type: Real
///
/// Default: Same as ELEC_EIGENVALUE_TOL
///
/// Example:
/// MD_ELEC_EIGENVALUE_TOL : 0.000007 eV
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "MD_ELEC_EIGENVALUE_TOL")]
#[serde(from = "MdElecEigenvalueTolRepr")] // Use intermediate repr for deserialization
pub struct MdElecEigenvalueTol {
    /// The eigenvalue tolerance value.
    pub value: f64,
    /// The optional unit of the energy value.
    pub unit: Option<EnergyUnit>,
}

/// Intermediate representation for deserializing `MdElecEigenvalueTol`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum MdElecEigenvalueTolRepr {
    /// Format: value unit
    WithUnit(f64, EnergyUnit),
    /// Format: value (default unit eV implied)
    Essential(f64),
}

impl From<MdElecEigenvalueTolRepr> for MdElecEigenvalueTol {
    fn from(repr: MdElecEigenvalueTolRepr) -> Self {
        match repr {
            MdElecEigenvalueTolRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            MdElecEigenvalueTolRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit (eV) implied
            },
        }
    }
}

impl ToCell for MdElecEigenvalueTol {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_ELEC_EIGENVALUE_TOL", self.to_cell_value())
    }
}

impl ToCellValue for MdElecEigenvalueTol {
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
    fn test_md_elec_eigenvalue_tol_serde() {
        // 1. Test Deserialization with unit
        let md_elec_eigenvalue_tol_with_unit_str = "MD_ELEC_EIGENVALUE_TOL : 0.000007 ev";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMdElecEigenvalueTolUnit {
            md_elec_eigenvalue_tol: MdElecEigenvalueTol,
        }

        let cell_file_result: Result<CellFileWithMdElecEigenvalueTolUnit, _> =
            from_str(md_elec_eigenvalue_tol_with_unit_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.md_elec_eigenvalue_tol.value - 0.000007).abs() < 1e-10);
        assert_eq!(
            cell_file.md_elec_eigenvalue_tol.unit,
            Some(EnergyUnit::ElectronVolt)
        );

        // 2. Test Deserialization without unit (default unit implied)
        // Note: Default logic is context-dependent. This just tests parsing a value without unit.
        let md_elec_eigenvalue_tol_default_str = "MD_ELEC_EIGENVALUE_TOL : 1e-6";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMdElecEigenvalueTolDefault {
            md_elec_eigenvalue_tol: MdElecEigenvalueTol,
        }

        let cell_file_default_result: Result<CellFileWithMdElecEigenvalueTolDefault, _> =
            from_str(md_elec_eigenvalue_tol_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.md_elec_eigenvalue_tol.value - 1e-6).abs() < f64::EPSILON);
        assert_eq!(cell_file_default.md_elec_eigenvalue_tol.unit, None);

        // 3. Test Serialization using ToCell (with unit)
        let md_elec_eigenvalue_tol_instance_with_unit = MdElecEigenvalueTol {
            value: 5e-7,
            unit: Some(EnergyUnit::Hartree),
        };
        let serialized_result_with_unit =
            to_string(&md_elec_eigenvalue_tol_instance_with_unit.to_cell());
        assert!(
            serialized_result_with_unit.is_ok(),
            "Serialization (with unit) failed: {:?}",
            serialized_result_with_unit.err()
        );
        let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        println!("Serialized MD_ELEC_EIGENVALUE_TOL (5e-7 ha): {serialized_string_with_unit}");
        assert!(serialized_string_with_unit.contains("MD_ELEC_EIGENVALUE_TOL"));
        assert!(
            serialized_string_with_unit.contains("5e-7")
                || serialized_string_with_unit.contains("0.0000005")
        );
        assert!(serialized_string_with_unit.contains("ha"));

        // 4. Test Serialization using ToCell (without unit)
        let md_elec_eigenvalue_tol_instance_no_unit = MdElecEigenvalueTol {
            value: 2e-6,
            unit: None,
        };
        let serialized_result_no_unit =
            to_string(&md_elec_eigenvalue_tol_instance_no_unit.to_cell());
        assert!(
            serialized_result_no_unit.is_ok(),
            "Serialization (no unit) failed: {:?}",
            serialized_result_no_unit.err()
        );
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized MD_ELEC_EIGENVALUE_TOL (2e-6, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("MD_ELEC_EIGENVALUE_TOL"));
        assert!(
            serialized_string_no_unit.contains("2e-6")
                || serialized_string_no_unit.contains("0.000002")
        );
        // Check that the unit string is not present (or is the default)
    }
}
