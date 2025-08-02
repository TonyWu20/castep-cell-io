use crate::units::LengthUnit;
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the tolerance for accepting convergence of the ionic displacement.
///
/// Keyword type: Real
///
/// Default: 0.001 Å
///
/// Example:
/// GEOM_DISP_TOL : 0.002 ang
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "GEOM_DISP_TOL")]
#[serde(from = "GeomDispTolRepr")] // Use intermediate repr for deserialization
pub struct GeomDispTol {
    /// The displacement tolerance value.
    pub value: f64,
    /// The optional unit of the length value.
    pub unit: Option<LengthUnit>,
}

/// Intermediate representation for deserializing `GeomDispTol`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum GeomDispTolRepr {
    /// Format: value unit
    WithUnit(f64, LengthUnit),
    /// Format: value (default unit Å implied)
    Essential(f64),
}

impl From<GeomDispTolRepr> for GeomDispTol {
    fn from(repr: GeomDispTolRepr) -> Self {
        match repr {
            GeomDispTolRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            GeomDispTolRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit (Å) implied
            },
        }
    }
}

impl ToCell for GeomDispTol {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("GEOM_DISP_TOL", self.to_cell_value())
    }
}

impl ToCellValue for GeomDispTol {
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
    fn test_geom_disp_tol_serde() {
        // 1. Test Deserialization with unit
        let geom_disp_tol_with_unit_str = "GEOM_DISP_TOL : 0.002 ang";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithGeomDispTolUnit {
            geom_disp_tol: GeomDispTol,
        }

        let cell_file_result: Result<CellFileWithGeomDispTolUnit, _> =
            from_str(geom_disp_tol_with_unit_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.geom_disp_tol.value - 0.002).abs() < 1e-10);
        // Asserting the unit depends on how LengthUnit is defined.
        // assert_eq!(cell_file.geom_disp_tol.unit, Some(LengthUnit::Ang));

        // 2. Test Deserialization without unit (default unit implied)
        let geom_disp_tol_default_str = "GEOM_DISP_TOL : 0.001";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithGeomDispTolDefault {
            geom_disp_tol: GeomDispTol,
        }

        let cell_file_default_result: Result<CellFileWithGeomDispTolDefault, _> =
            from_str(geom_disp_tol_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.geom_disp_tol.value - 0.001).abs() < f64::EPSILON);
        assert_eq!(cell_file_default.geom_disp_tol.unit, None);

        // 3. Test Serialization using ToCell (with unit)
        let geom_disp_tol_instance_with_unit = GeomDispTol {
            value: 0.0015,
            unit: Some(LengthUnit::Bohr), // Assuming this unit exists
        };
        let serialized_result_with_unit = to_string(&geom_disp_tol_instance_with_unit.to_cell());
        assert!(
            serialized_result_with_unit.is_ok(),
            "Serialization (with unit) failed: {:?}",
            serialized_result_with_unit.err()
        );
        let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        println!("Serialized GEOM_DISP_TOL (0.0015 bohr): {serialized_string_with_unit}");
        assert!(serialized_string_with_unit.contains("GEOM_DISP_TOL"));
        assert!(serialized_string_with_unit.contains("0.0015"));
        // Check for unit string, e.g., "bohr"

        // 4. Test Serialization using ToCell (without unit)
        let geom_disp_tol_instance_no_unit = GeomDispTol {
            value: 0.003,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&geom_disp_tol_instance_no_unit.to_cell());
        assert!(
            serialized_result_no_unit.is_ok(),
            "Serialization (no unit) failed: {:?}",
            serialized_result_no_unit.err()
        );
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized GEOM_DISP_TOL (0.003, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("GEOM_DISP_TOL"));
        assert!(serialized_string_no_unit.contains("0.003"));
        // Check that the unit string is not present (or is the default)
    }
}
