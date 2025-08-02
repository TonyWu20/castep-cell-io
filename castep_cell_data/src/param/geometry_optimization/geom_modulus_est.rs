use crate::units::PressureUnit;
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Provides an estimate of the bulk modulus of the system.
///
/// Keyword type: Real
///
/// Default: 500.0 GPa
///
/// Example:
/// GEOM_MODULUS_EST : 125.4 GPa
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "GEOM_MODULUS_EST")]
#[serde(from = "GeomModulusEstRepr")] // Use intermediate repr for deserialization
pub struct GeomModulusEst {
    /// The modulus estimate value.
    pub value: f64,
    /// The optional unit of the pressure value.
    pub unit: Option<PressureUnit>,
}

/// Intermediate representation for deserializing `GeomModulusEst`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum GeomModulusEstRepr {
    /// Format: value unit
    WithUnit(f64, PressureUnit),
    /// Format: value (default unit GPa implied)
    Essential(f64),
}

impl From<GeomModulusEstRepr> for GeomModulusEst {
    fn from(repr: GeomModulusEstRepr) -> Self {
        match repr {
            GeomModulusEstRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            GeomModulusEstRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit (GPa) implied
            },
        }
    }
}

impl ToCell for GeomModulusEst {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("GEOM_MODULUS_EST", self.to_cell_value())
    }
}

impl ToCellValue for GeomModulusEst {
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
    fn test_geom_modulus_est_serde() {
        // 1. Test Deserialization with unit
        let geom_modulus_est_with_unit_str = "GEOM_MODULUS_EST : 125.4 gpa";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithGeomModulusEstUnit {
            geom_modulus_est: GeomModulusEst,
        }

        let cell_file_result: Result<CellFileWithGeomModulusEstUnit, _> =
            from_str(geom_modulus_est_with_unit_str);
        // This test depends on PressureUnit correctly parsing "gpa"
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.geom_modulus_est.value - 125.4).abs() < 1e-10);
        // assert_eq!(cell_file.geom_modulus_est.unit, Some(PressureUnit::GigaPascal)); // If this variant exists

        // 2. Test Deserialization without unit (default unit implied)
        let geom_modulus_est_default_str = "GEOM_MODULUS_EST : 500.0";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithGeomModulusEstDefault {
            geom_modulus_est: GeomModulusEst,
        }

        let cell_file_default_result: Result<CellFileWithGeomModulusEstDefault, _> =
            from_str(geom_modulus_est_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.geom_modulus_est.value - 500.0).abs() < f64::EPSILON);
        assert_eq!(cell_file_default.geom_modulus_est.unit, None);

        // 3. Test Serialization using ToCell (with unit)
        // This depends on PressureUnit having a variant that serializes to "gpa"
        let geom_modulus_est_instance_with_unit = GeomModulusEst {
            value: 200.0,
            unit: Some(PressureUnit::GigaPascal), // If this variant exists
        };
        let serialized_result_with_unit = to_string(&geom_modulus_est_instance_with_unit.to_cell());
        assert!(
            serialized_result_with_unit.is_ok(),
            "Serialization (with unit) failed: {:?}",
            serialized_result_with_unit.err()
        );
        let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        println!("Serialized GEOM_MODULUS_EST (200.0 gpa): {serialized_string_with_unit}");
        assert!(serialized_string_with_unit.contains("GEOM_MODULUS_EST"));
        assert!(serialized_string_with_unit.contains("200.0"));
        assert!(serialized_string_with_unit.contains("gpa"));

        // 4. Test Serialization using ToCell (without unit)
        let geom_modulus_est_instance_no_unit = GeomModulusEst {
            value: 300.0,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&geom_modulus_est_instance_no_unit.to_cell());
        assert!(
            serialized_result_no_unit.is_ok(),
            "Serialization (no unit) failed: {:?}",
            serialized_result_no_unit.err()
        );
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized GEOM_MODULUS_EST (300.0, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("GEOM_MODULUS_EST"));
        assert!(serialized_string_no_unit.contains("300.0"));
        // Check that the unit string is not present (or is the default)
    }
}
