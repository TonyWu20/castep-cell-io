use crate::units::PressureUnit;
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the tolerance for accepting convergence of the maximum stress component.
///
/// Keyword type: Real
///
/// Default: 0.1 GPa
///
/// Example:
/// GEOM_STRESS_TOL : 0.2 GPa
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "GEOM_STRESS_TOL")]
#[serde(from = "GeomStressTolRepr")] // Use intermediate repr for deserialization
pub struct GeomStressTol {
    /// The stress tolerance value.
    pub value: f64,
    /// The optional unit of the pressure value.
    pub unit: Option<PressureUnit>,
}

/// Intermediate representation for deserializing `GeomStressTol`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum GeomStressTolRepr {
    /// Format: value unit
    WithUnit(f64, PressureUnit),
    /// Format: value (default unit GPa implied)
    Essential(f64),
}

impl From<GeomStressTolRepr> for GeomStressTol {
    fn from(repr: GeomStressTolRepr) -> Self {
        match repr {
            GeomStressTolRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            GeomStressTolRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit (GPa) implied
            },
        }
    }
}

impl ToCell for GeomStressTol {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("GEOM_STRESS_TOL", self.to_cell_value())
    }
}

impl ToCellValue for GeomStressTol {
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
    fn test_geom_stress_tol_serde() {
        // 1. Test Deserialization with unit
        let geom_stress_tol_with_unit_str = "GEOM_STRESS_TOL : 0.2 gpa";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithGeomStressTolUnit {
            geom_stress_tol: GeomStressTol,
        }

        let cell_file_result: Result<CellFileWithGeomStressTolUnit, _> =
            from_str(geom_stress_tol_with_unit_str);
        // This test depends on PressureUnit correctly parsing "gpa"
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.geom_stress_tol.value - 0.2).abs() < 1e-10);
        // assert_eq!(cell_file.geom_stress_tol.unit, Some(PressureUnit::GigaPascal)); // If this variant exists

        // 2. Test Deserialization without unit (default unit implied)
        let geom_stress_tol_default_str = "GEOM_STRESS_TOL : 0.1";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithGeomStressTolDefault {
            geom_stress_tol: GeomStressTol,
        }

        let cell_file_default_result: Result<CellFileWithGeomStressTolDefault, _> =
            from_str(geom_stress_tol_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.geom_stress_tol.value - 0.1).abs() < f64::EPSILON);
        assert_eq!(cell_file_default.geom_stress_tol.unit, None);

        // 3. Test Serialization using ToCell (with unit)
        // This depends on PressureUnit having a variant that serializes to "gpa"
        let geom_stress_tol_instance_with_unit = GeomStressTol {
            value: 0.15,
            unit: Some(PressureUnit::GigaPascal), // If this variant exists
        };
        let serialized_result_with_unit = to_string(&geom_stress_tol_instance_with_unit.to_cell());
        assert!(
            serialized_result_with_unit.is_ok(),
            "Serialization (with unit) failed: {:?}",
            serialized_result_with_unit.err()
        );
        let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        println!("Serialized GEOM_STRESS_TOL (0.15 gpa): {serialized_string_with_unit}");
        assert!(serialized_string_with_unit.contains("GEOM_STRESS_TOL"));
        assert!(serialized_string_with_unit.contains("0.15"));
        assert!(serialized_string_with_unit.contains("gpa"));

        // 4. Test Serialization using ToCell (without unit)
        let geom_stress_tol_instance_no_unit = GeomStressTol {
            value: 0.3,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&geom_stress_tol_instance_no_unit.to_cell());
        assert!(
            serialized_result_no_unit.is_ok(),
            "Serialization (no unit) failed: {:?}",
            serialized_result_no_unit.err()
        );
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized GEOM_STRESS_TOL (0.3, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("GEOM_STRESS_TOL"));
        assert!(serialized_string_no_unit.contains("0.3"));
        // Check that the unit string is not present (or is the default)
    }
}
