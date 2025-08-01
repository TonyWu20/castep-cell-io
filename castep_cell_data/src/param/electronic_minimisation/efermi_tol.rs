// File: electronic_minimisation/efemi_tol.rs
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};
// Assuming EnergyUnit exists in units module
use crate::units::EnergyUnit;

/// Controls the tolerance for accepting convergence of the Fermi-energy.
///
/// Keyword type: Real
///
/// Default: 0.1 × ELEC_EIGENVALUE_TOL
///
/// Example:
/// EFERMI_TOL : 0.0000007 eV
/// EFERMI_TOL : 0.0000007 (uses default unit, likely eV)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "EFERMI_TOL")]
#[serde(from = "EFermiTolRepr")] // Use intermediate repr for deserialization
pub struct EFermiTol {
    /// The Fermi energy tolerance value.
    pub value: f64,
    /// The optional unit of the energy value.
    /// If None, a default unit (likely eV) is implied.
    pub unit: Option<EnergyUnit>,
}

/// Intermediate representation for deserializing `EFermiTol`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum EFermiTolRepr {
    /// Format: value unit
    WithUnit(f64, EnergyUnit),
    /// Format: value (default unit implied)
    Essential(f64),
}

impl From<EFermiTolRepr> for EFermiTol {
    fn from(repr: EFermiTolRepr) -> Self {
        match repr {
            EFermiTolRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            EFermiTolRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit implied
            },
        }
    }
}

impl ToCell for EFermiTol {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("EFERMI_TOL", self.to_cell_value())
    }
}

impl ToCellValue for EFermiTol {
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
    fn test_efermi_tol_serde() {
        // 1. Test Deserialization with unit
        let efermi_tol_with_unit_str = "EFERMI_TOL : 0.0000007 ev";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithEFermiTolUnit {
            efermi_tol: EFermiTol,
        }

        let cell_file_result: Result<CellFileWithEFermiTolUnit, _> =
            from_str(efermi_tol_with_unit_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.efermi_tol.value - 0.0000007).abs() < 1e-10);
        assert_eq!(cell_file.efermi_tol.unit, Some(EnergyUnit::ElectronVolt));

        // 2. Test Deserialization without unit (default unit implied)
        let efermi_tol_default_str = "EFERMI_TOL : 0.0000001";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithEFermiTolDefault {
            efermi_tol: EFermiTol,
        }

        let cell_file_default_result: Result<CellFileWithEFermiTolDefault, _> =
            from_str(efermi_tol_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.efermi_tol.value - 0.0000001).abs() < 1e-10);
        assert_eq!(cell_file_default.efermi_tol.unit, None);

        // 3. Test Serialization using ToCell (with unit)
        let efermi_tol_instance_with_unit = EFermiTol {
            value: 1e-7,
            unit: Some(EnergyUnit::Hartree),
        };
        let serialized_result_with_unit = to_string(&efermi_tol_instance_with_unit.to_cell());
        assert!(
            serialized_result_with_unit.is_ok(),
            "Serialization (with unit) failed: {:?}",
            serialized_result_with_unit.err()
        );
        let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        println!("Serialized EFERMI_TOL (1e-7 ha): {serialized_string_with_unit}");
        assert!(serialized_string_with_unit.contains("EFERMI_TOL"));
        assert!(
            serialized_string_with_unit.contains("1e-7")
                || serialized_string_with_unit.contains("0.0000001")
        );
        assert!(serialized_string_with_unit.contains("ha"));

        // 4. Test Serialization using ToCell (without unit)
        let efermi_tol_instance_no_unit = EFermiTol {
            value: 2e-7,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&efermi_tol_instance_no_unit.to_cell());
        assert!(
            serialized_result_no_unit.is_ok(),
            "Serialization (no unit) failed: {:?}",
            serialized_result_no_unit.err()
        );
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized EFERMI_TOL (2e-7, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("EFERMI_TOL"));
        assert!(
            serialized_string_no_unit.contains("2e-7")
                || serialized_string_no_unit.contains("0.0000002")
        );
        // Check that the unit string is not present
        assert!(!serialized_string_no_unit.contains("ev"));
        assert!(!serialized_string_no_unit.contains("ha"));
        // ... (add checks for other unit strings if necessary)
    }
}
