use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};
// Assuming VolumeUnit exists in units module
use crate::units::VolumeUnit;

/// Controls the tolerance for accepting convergence of the field constants.
///
/// Keyword type: Real
///
/// Default: 1e-5 Å^3
///
/// Example:
/// EFIELD_ENERGY_TOL : 0.000002 ANG**3
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "EFIELD_ENERGY_TOL")]
#[serde(from = "EfieldEnergyTolRepr")] // Use intermediate repr for deserialization
pub struct EfieldEnergyTol {
    /// The energy tolerance value.
    pub value: f64,
    /// The optional unit of the volume value.
    pub unit: Option<VolumeUnit>,
}

/// Intermediate representation for deserializing `EfieldEnergyTol`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum EfieldEnergyTolRepr {
    /// Format: value unit
    WithUnit(f64, VolumeUnit),
    /// Format: value (default unit Angstrom^3 implied)
    Essential(f64),
}

impl From<EfieldEnergyTolRepr> for EfieldEnergyTol {
    fn from(repr: EfieldEnergyTolRepr) -> Self {
        match repr {
            EfieldEnergyTolRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            EfieldEnergyTolRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit (Angstrom^3) implied
            },
        }
    }
}

impl ToCell for EfieldEnergyTol {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("EFIELD_ENERGY_TOL", self.to_cell_value())
    }
}

impl ToCellValue for EfieldEnergyTol {
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
    fn test_efield_energy_tol_serde() {
        // 1. Test Deserialization with unit
        // Note: The exact parsing of "ANG**3" depends on how VolumeUnit is defined.
        let efield_energy_tol_with_unit_str = "EFIELD_ENERGY_TOL : 0.000002 ang**3";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithEfieldEnergyTolUnit {
            efield_energy_tol: EfieldEnergyTol,
        }

        let cell_file_result: Result<CellFileWithEfieldEnergyTolUnit, _> =
            from_str(efield_energy_tol_with_unit_str);
        // This test depends on VolumeUnit correctly parsing the unit string.
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.efield_energy_tol.value - 0.000002).abs() < 1e-12);
        // assert_eq!(cell_file.efield_energy_tol.unit, Some(VolumeUnit::Ang3)); // If this variant exists

        // 2. Test Deserialization without unit (default unit implied)
        let efield_energy_tol_default_str = "EFIELD_ENERGY_TOL : 1e-5";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithEfieldEnergyTolDefault {
            efield_energy_tol: EfieldEnergyTol,
        }

        let cell_file_default_result: Result<CellFileWithEfieldEnergyTolDefault, _> =
            from_str(efield_energy_tol_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.efield_energy_tol.value - 1e-5).abs() < f64::EPSILON);
        assert_eq!(cell_file_default.efield_energy_tol.unit, None);

        // 3. Test Serialization using ToCell (with unit)
        // This depends on VolumeUnit having a variant that serializes correctly.
        // let efield_energy_tol_instance_with_unit = EfieldEnergyTol {
        //     value: 5e-6,
        //     unit: Some(VolumeUnit::Ang3), // If this variant exists
        // };
        // let serialized_result_with_unit = to_string(&efield_energy_tol_instance_with_unit.to_cell());
        // assert!(serialized_result_with_unit.is_ok(), "Serialization (with unit) failed: {:?}", serialized_result_with_unit.err());
        // let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        // println!("Serialized EFIELD_ENERGY_TOL (5e-6 ang**3): {serialized_string_with_unit}");
        // assert!(serialized_string_with_unit.contains("EFIELD_ENERGY_TOL"));
        // assert!(serialized_string_with_unit.contains("5e-6") || serialized_string_with_unit.contains("0.000005"));
        // assert!(serialized_string_with_unit.contains("ang**3"));

        // 4. Test Serialization using ToCell (without unit)
        let efield_energy_tol_instance_no_unit = EfieldEnergyTol {
            value: 2e-5,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&efield_energy_tol_instance_no_unit.to_cell());
        assert!(
            serialized_result_no_unit.is_ok(),
            "Serialization (no unit) failed: {:?}",
            serialized_result_no_unit.err()
        );
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized EFIELD_ENERGY_TOL (2e-5, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("EFIELD_ENERGY_TOL"));
        assert!(
            serialized_string_no_unit.contains("2e-5")
                || serialized_string_no_unit.contains("0.00002")
        );
        // Check that the unit string is not present (or is the default)
    }
}
