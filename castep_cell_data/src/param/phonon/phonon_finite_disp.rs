use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};
// Assuming LengthUnit exists in units module
use crate::units::LengthUnit;

/// Specifies the amplitude of the ionic perturbation for finite displacement phonons.
///
/// Keyword type: Real
///
/// Default: 0.01 Bohr
///
/// Example:
/// PHONON_FINITE_DISP : 0.01 ANG
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "PHONON_FINITE_DISP")]
#[serde(from = "PhononFiniteDispRepr")] // Use intermediate repr for deserialization
pub struct PhononFiniteDisp {
    /// The displacement amplitude value.
    pub value: f64,
    /// The optional unit of the length value.
    pub unit: Option<LengthUnit>,
}

/// Intermediate representation for deserializing `PhononFiniteDisp`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum PhononFiniteDispRepr {
    /// Format: value unit
    WithUnit(f64, LengthUnit),
    /// Format: value (default unit Bohr implied)
    Essential(f64),
}

impl From<PhononFiniteDispRepr> for PhononFiniteDisp {
    fn from(repr: PhononFiniteDispRepr) -> Self {
        match repr {
            PhononFiniteDispRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            PhononFiniteDispRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit (Bohr) implied
            },
        }
    }
}

impl ToCell for PhononFiniteDisp {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PHONON_FINITE_DISP", self.to_cell_value())
    }
}

impl ToCellValue for PhononFiniteDisp {
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
    fn test_phonon_finite_disp_serde() {
        // 1. Test Deserialization with unit
        let phonon_finite_disp_with_unit_str = "PHONON_FINITE_DISP : 0.01 ang";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPhononFiniteDispUnit {
            phonon_finite_disp: PhononFiniteDisp,
        }

        let cell_file_result: Result<CellFileWithPhononFiniteDispUnit, _> =
            from_str(phonon_finite_disp_with_unit_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.phonon_finite_disp.value - 0.01).abs() < f64::EPSILON);
        // Asserting the unit depends on how LengthUnit is defined.
        // assert_eq!(cell_file.phonon_finite_disp.unit, Some(LengthUnit::Ang));

        // 2. Test Deserialization without unit (default unit implied)
        let phonon_finite_disp_default_str = "PHONON_FINITE_DISP : 0.01";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPhononFiniteDispDefault {
            phonon_finite_disp: PhononFiniteDisp,
        }

        let cell_file_default_result: Result<CellFileWithPhononFiniteDispDefault, _> =
            from_str(phonon_finite_disp_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.phonon_finite_disp.value - 0.01).abs() < f64::EPSILON);
        assert_eq!(cell_file_default.phonon_finite_disp.unit, None);

        // 3. Test Serialization using ToCell (with unit)
        let phonon_finite_disp_instance_with_unit = PhononFiniteDisp {
            value: 0.005,
            unit: Some(LengthUnit::Bohr), // Assuming this unit exists
        };
        let serialized_result_with_unit =
            to_string(&phonon_finite_disp_instance_with_unit.to_cell());
        assert!(
            serialized_result_with_unit.is_ok(),
            "Serialization (with unit) failed: {:?}",
            serialized_result_with_unit.err()
        );
        let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        println!("Serialized PHONON_FINITE_DISP (0.005 bohr): {serialized_string_with_unit}");
        assert!(serialized_string_with_unit.contains("PHONON_FINITE_DISP"));
        assert!(serialized_string_with_unit.contains("0.005"));
        // Check for unit string, e.g., "bohr"

        // 4. Test Serialization using ToCell (without unit)
        let phonon_finite_disp_instance_no_unit = PhononFiniteDisp {
            value: 0.02,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&phonon_finite_disp_instance_no_unit.to_cell());
        assert!(
            serialized_result_no_unit.is_ok(),
            "Serialization (no unit) failed: {:?}",
            serialized_result_no_unit.err()
        );
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized PHONON_FINITE_DISP (0.02, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("PHONON_FINITE_DISP"));
        assert!(serialized_string_no_unit.contains("0.02"));
        // Check that the unit string is not present (or is the default)
    }
}
