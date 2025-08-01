use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};
// Assuming EnergyUnit exists in units module
use crate::units::EnergyUnit;

/// Controls the tolerance for accepting convergence of the total energy in an electronic minimization.
///
/// Keyword type: Real
///
/// Default: 1e-5 eV per atom
///
/// Example:
/// ELEC_ENERGY_TOL : 0.00007 eV
/// ELEC_ENERGY_TOL : 0.00007 (uses default unit, likely eV)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "ELEC_ENERGY_TOL")]
#[serde(from = "ElecEnergyTolRepr")] // Use intermediate repr for deserialization
pub struct ElecEnergyTol {
    /// The energy tolerance value.
    pub value: f64,
    /// The optional unit of the energy value.
    /// If None, a default unit (likely eV) is implied.
    pub unit: Option<EnergyUnit>,
}

/// Intermediate representation for deserializing `ElecEnergyTol`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ElecEnergyTolRepr {
    /// Format: value unit
    WithUnit(f64, EnergyUnit),
    /// Format: value (default unit implied)
    Essential(f64),
}

impl From<ElecEnergyTolRepr> for ElecEnergyTol {
    fn from(repr: ElecEnergyTolRepr) -> Self {
        match repr {
            ElecEnergyTolRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            ElecEnergyTolRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit implied
            },
        }
    }
}

impl ToCell for ElecEnergyTol {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("ELEC_ENERGY_TOL", self.to_cell_value())
    }
}

impl ToCellValue for ElecEnergyTol {
    fn to_cell_value(&self) -> CellValue {
        // Create a CellValue::Array containing the value and optionally the unit
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
    fn test_elec_energy_tol_serde() {
        // 1. Test Deserialization with unit
        let elec_energy_tol_with_unit_str = "ELEC_ENERGY_TOL : 0.00007 ev";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithElecEnergyTolUnit {
            elec_energy_tol: ElecEnergyTol,
        }

        let cell_file_result: Result<CellFileWithElecEnergyTolUnit, _> =
            from_str(elec_energy_tol_with_unit_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.elec_energy_tol.value - 0.00007).abs() < 1e-10);
        assert_eq!(
            cell_file.elec_energy_tol.unit,
            Some(EnergyUnit::ElectronVolt)
        );

        // 2. Test Deserialization without unit (default unit implied)
        let elec_energy_tol_default_str = "ELEC_ENERGY_TOL : 0.00001";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithElecEnergyTolDefault {
            elec_energy_tol: ElecEnergyTol,
        }

        let cell_file_default_result: Result<CellFileWithElecEnergyTolDefault, _> =
            from_str(elec_energy_tol_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.elec_energy_tol.value - 0.00001).abs() < 1e-10);
        assert_eq!(cell_file_default.elec_energy_tol.unit, None);

        // 3. Test Serialization using ToCell (with unit)
        let elec_energy_tol_instance_with_unit = ElecEnergyTol {
            value: 1e-5,
            unit: Some(EnergyUnit::Hartree),
        };
        let serialized_result_with_unit = to_string(&elec_energy_tol_instance_with_unit.to_cell());
        assert!(
            serialized_result_with_unit.is_ok(),
            "Serialization (with unit) failed: {:?}",
            serialized_result_with_unit.err()
        );
        let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        println!("Serialized ELEC_ENERGY_TOL (1e-5 ha): {serialized_string_with_unit}");
        assert!(serialized_string_with_unit.contains("ELEC_ENERGY_TOL"));
        assert!(
            serialized_string_with_unit.contains("1e-5")
                || serialized_string_with_unit.contains("0.00001")
        );
        assert!(serialized_string_with_unit.contains("ha"));

        // 4. Test Serialization using ToCell (without unit)
        let elec_energy_tol_instance_no_unit = ElecEnergyTol {
            value: 2e-5,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&elec_energy_tol_instance_no_unit.to_cell());
        assert!(
            serialized_result_no_unit.is_ok(),
            "Serialization (no unit) failed: {:?}",
            serialized_result_no_unit.err()
        );
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized ELEC_ENERGY_TOL (2e-5, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("ELEC_ENERGY_TOL"));
        assert!(
            serialized_string_no_unit.contains("2e-5")
                || serialized_string_no_unit.contains("0.00002")
        );
        // Check that the unit string is not present
        assert!(!serialized_string_no_unit.contains("ev"));
        assert!(!serialized_string_no_unit.contains("ha"));
        // ... (add checks for other unit strings if necessary)
    }
}
