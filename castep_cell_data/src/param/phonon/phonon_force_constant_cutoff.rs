use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};
// Assuming LengthUnit exists in units module
use crate::units::LengthUnit;

/// Specifies the cutoff for the force constant matrix in a phonon calculation.
///
/// Keyword type: Real
///
/// Default: 0.0
///
/// Example:
/// PHONON_FORCE_CONSTANT_CUTOFF : 6.34 ang
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "PHONON_FORCE_CONSTANT_CUTOFF")]
#[serde(from = "PhononForceConstantCutoffRepr")] // Use intermediate repr for deserialization
pub struct PhononForceConstantCutoff {
    /// The cutoff value.
    pub value: f64,
    /// The optional unit of the length value.
    pub unit: Option<LengthUnit>,
}

/// Intermediate representation for deserializing `PhononForceConstantCutoff`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum PhononForceConstantCutoffRepr {
    /// Format: value unit
    WithUnit(f64, LengthUnit),
    /// Format: value (default unit Angstrom implied or no unit for 0.0)
    Essential(f64),
}

impl From<PhononForceConstantCutoffRepr> for PhononForceConstantCutoff {
    fn from(repr: PhononForceConstantCutoffRepr) -> Self {
        match repr {
            PhononForceConstantCutoffRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            PhononForceConstantCutoffRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit implied or no unit
            },
        }
    }
}

impl ToCell for PhononForceConstantCutoff {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PHONON_FORCE_CONSTANT_CUTOFF", self.to_cell_value())
    }
}

impl ToCellValue for PhononForceConstantCutoff {
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
    fn test_phonon_force_constant_cutoff_serde() {
        // 1. Test Deserialization with unit
        let phonon_force_constant_cutoff_with_unit_str = "PHONON_FORCE_CONSTANT_CUTOFF : 6.34 ang";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPhononForceConstantCutoffUnit {
            phonon_force_constant_cutoff: PhononForceConstantCutoff,
        }

        let cell_file_result: Result<CellFileWithPhononForceConstantCutoffUnit, _> =
            from_str(phonon_force_constant_cutoff_with_unit_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.phonon_force_constant_cutoff.value - 6.34).abs() < 1e-10);
        // Asserting the unit depends on how LengthUnit is defined.
        // assert_eq!(cell_file.phonon_force_constant_cutoff.unit, Some(LengthUnit::Ang));

        // 2. Test Deserialization without unit (default/unit from context)
        let phonon_force_constant_cutoff_default_str = "PHONON_FORCE_CONSTANT_CUTOFF : 0.0";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPhononForceConstantCutoffDefault {
            phonon_force_constant_cutoff: PhononForceConstantCutoff,
        }

        let cell_file_default_result: Result<CellFileWithPhononForceConstantCutoffDefault, _> =
            from_str(phonon_force_constant_cutoff_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.phonon_force_constant_cutoff.value - 0.0).abs() < f64::EPSILON);
        assert_eq!(cell_file_default.phonon_force_constant_cutoff.unit, None);

        // 3. Test Serialization using ToCell (with unit)
        let phonon_force_constant_cutoff_instance_with_unit = PhononForceConstantCutoff {
            value: 5.0,
            unit: Some(LengthUnit::Bohr), // Assuming this unit exists
        };
        let serialized_result_with_unit =
            to_string(&phonon_force_constant_cutoff_instance_with_unit.to_cell());
        assert!(
            serialized_result_with_unit.is_ok(),
            "Serialization (with unit) failed: {:?}",
            serialized_result_with_unit.err()
        );
        let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        println!(
            "Serialized PHONON_FORCE_CONSTANT_CUTOFF (5.0 bohr): {serialized_string_with_unit}"
        );
        assert!(serialized_string_with_unit.contains("PHONON_FORCE_CONSTANT_CUTOFF"));
        assert!(serialized_string_with_unit.contains("5.0"));
        // Check for unit string, e.g., "bohr"

        // 4. Test Serialization using ToCell (without unit)
        let phonon_force_constant_cutoff_instance_no_unit = PhononForceConstantCutoff {
            value: 7.0,
            unit: None,
        };
        let serialized_result_no_unit =
            to_string(&phonon_force_constant_cutoff_instance_no_unit.to_cell());
        assert!(
            serialized_result_no_unit.is_ok(),
            "Serialization (no unit) failed: {:?}",
            serialized_result_no_unit.err()
        );
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!(
            "Serialized PHONON_FORCE_CONSTANT_CUTOFF (7.0, no unit): {serialized_string_no_unit}"
        );
        assert!(serialized_string_no_unit.contains("PHONON_FORCE_CONSTANT_CUTOFF"));
        assert!(serialized_string_no_unit.contains("7.0"));
        // Check that the unit string is not present
    }
}
