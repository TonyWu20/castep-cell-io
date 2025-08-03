use crate::units::EnergyUnit;
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the tolerance for accepting convergence of the first-order perturbed wavefunctions.
///
/// Keyword type: Real
///
/// Default:
/// 1e-12 when MAGRES_METHOD : crystal
/// 1e-9 when MAGRES_METHOD : molecular
///
/// Example:
/// MAGRES_CONV_TOL = 0.00007 eV
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "MAGRES_CONV_TOL")]
#[serde(from = "MagresConvTolRepr")] // Use intermediate repr for deserialization
pub struct MagresConvTol {
    /// The convergence tolerance value.
    pub value: f64,
    /// The optional unit of the energy value.
    pub unit: Option<EnergyUnit>,
}

/// Intermediate representation for deserializing `MagresConvTol`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum MagresConvTolRepr {
    /// Format: value unit
    WithUnit(f64, EnergyUnit),
    /// Format: value (default unit eV implied)
    Essential(f64),
}

impl From<MagresConvTolRepr> for MagresConvTol {
    fn from(repr: MagresConvTolRepr) -> Self {
        match repr {
            MagresConvTolRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            MagresConvTolRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit (eV) implied
            },
        }
    }
}

impl ToCell for MagresConvTol {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MAGRES_CONV_TOL", self.to_cell_value())
    }
}

impl ToCellValue for MagresConvTol {
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
    fn test_magres_conv_tol_serde() {
        // 1. Test Deserialization with unit
        let magres_conv_tol_with_unit_str = "MAGRES_CONV_TOL : 0.00007 ev";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMagresConvTolUnit {
            magres_conv_tol: MagresConvTol,
        }

        let cell_file_result: Result<CellFileWithMagresConvTolUnit, _> =
            from_str(magres_conv_tol_with_unit_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.magres_conv_tol.value - 0.00007).abs() < 1e-10);
        assert_eq!(
            cell_file.magres_conv_tol.unit,
            Some(EnergyUnit::ElectronVolt)
        );

        // 2. Test Deserialization without unit (default unit implied)
        // Note: Default logic is context-dependent. This just tests parsing a value without unit.
        let magres_conv_tol_default_str = "MAGRES_CONV_TOL : 1e-12";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMagresConvTolDefault {
            magres_conv_tol: MagresConvTol,
        }

        let cell_file_default_result: Result<CellFileWithMagresConvTolDefault, _> =
            from_str(magres_conv_tol_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.magres_conv_tol.value - 1e-12).abs() < f64::EPSILON);
        assert_eq!(cell_file_default.magres_conv_tol.unit, None);

        // 3. Test Serialization using ToCell (with unit)
        let magres_conv_tol_instance_with_unit = MagresConvTol {
            value: 5e-10,
            unit: Some(EnergyUnit::Hartree),
        };
        let serialized_result_with_unit = to_string(&magres_conv_tol_instance_with_unit.to_cell());
        assert!(
            serialized_result_with_unit.is_ok(),
            "Serialization (with unit) failed: {:?}",
            serialized_result_with_unit.err()
        );
        let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        println!("Serialized MAGRES_CONV_TOL (5e-10 ha): {serialized_string_with_unit}");
        assert!(serialized_string_with_unit.contains("MAGRES_CONV_TOL"));
        assert!(
            serialized_string_with_unit.contains("5e-10")
                || serialized_string_with_unit.contains("0.0000000005")
        );
        assert!(serialized_string_with_unit.contains("ha"));

        // 4. Test Serialization using ToCell (without unit)
        let magres_conv_tol_instance_no_unit = MagresConvTol {
            value: 2e-11,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&magres_conv_tol_instance_no_unit.to_cell());
        assert!(
            serialized_result_no_unit.is_ok(),
            "Serialization (no unit) failed: {:?}",
            serialized_result_no_unit.err()
        );
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized MAGRES_CONV_TOL (2e-11, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("MAGRES_CONV_TOL"));
        assert!(
            serialized_string_no_unit.contains("2e-11")
                || serialized_string_no_unit.contains("0.00000000002")
        );
        // Check that the unit string is not present (or is the default)
    }
}
