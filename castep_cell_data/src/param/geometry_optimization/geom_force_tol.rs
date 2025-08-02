use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};
// Assuming a combined ForceUnit or separate units might be needed.
// Based on the example "0.05 eV Å -1" and "0.07 ev/ang", it seems like a specific unit type
// combining energy and inverse length is implied, or ForceUnit includes these combinations.
// For now, let's assume ForceUnit handles this, or we create a specific unit if needed.
// The documentation mentions "eV Å -1" and "ev/ang". Let's assume ForceUnit covers this.
use crate::units::ForceUnit; // This needs to include ev/ang if that's a valid variant

/// Controls the tolerance for accepting convergence of the ionic force.
///
/// Keyword type: Real
///
/// Default: 0.05 eV/Å
///
/// Example:
/// GEOM_FORCE_TOL : 0.07 ev/ang
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "GEOM_FORCE_TOL")]
#[serde(from = "GeomForceTolRepr")] // Use intermediate repr for deserialization
pub struct GeomForceTol {
    /// The force tolerance value.
    pub value: f64,
    /// The optional unit of the force value.
    pub unit: Option<ForceUnit>,
}

/// Intermediate representation for deserializing `GeomForceTol`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum GeomForceTolRepr {
    /// Format: value unit
    WithUnit(f64, ForceUnit),
    /// Format: value (default unit eV/Å implied)
    Essential(f64),
}

impl From<GeomForceTolRepr> for GeomForceTol {
    fn from(repr: GeomForceTolRepr) -> Self {
        match repr {
            GeomForceTolRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            GeomForceTolRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit (eV/Å) implied
            },
        }
    }
}

impl ToCell for GeomForceTol {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("GEOM_FORCE_TOL", self.to_cell_value())
    }
}

impl ToCellValue for GeomForceTol {
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
    fn test_geom_force_tol_serde() {
        // 1. Test Deserialization with unit (assuming ev/ang is a valid ForceUnit variant)
        let geom_force_tol_with_unit_str = "GEOM_FORCE_TOL : 0.07 ev/ang";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithGeomForceTolUnit {
            geom_force_tol: GeomForceTol,
        }

        let cell_file_result: Result<CellFileWithGeomForceTolUnit, _> =
            from_str(geom_force_tol_with_unit_str);
        // This test depends on ForceUnit correctly parsing "ev/ang"
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.geom_force_tol.value - 0.07).abs() < 1e-10);
        // assert_eq!(cell_file.geom_force_tol.unit, Some(ForceUnit::EvPerAng)); // If this variant exists

        // 2. Test Deserialization without unit (default unit implied)
        let geom_force_tol_default_str = "GEOM_FORCE_TOL : 0.05";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithGeomForceTolDefault {
            geom_force_tol: GeomForceTol,
        }

        let cell_file_default_result: Result<CellFileWithGeomForceTolDefault, _> =
            from_str(geom_force_tol_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.geom_force_tol.value - 0.05).abs() < f64::EPSILON);
        assert_eq!(cell_file_default.geom_force_tol.unit, None);

        // 3. Test Serialization using ToCell (with unit)
        // This depends on ForceUnit having a variant that serializes to something like "ev/ang"
        let geom_force_tol_instance_with_unit = GeomForceTol {
            value: 0.06,
            unit: Some(ForceUnit::EvPerAng), // If this variant exists
        };
        let serialized_result_with_unit = to_string(&geom_force_tol_instance_with_unit.to_cell());
        assert!(
            serialized_result_with_unit.is_ok(),
            "Serialization (with unit) failed: {:?}",
            serialized_result_with_unit.err()
        );
        let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        println!("Serialized GEOM_FORCE_TOL (0.06 ev/ang): {serialized_string_with_unit}");
        assert!(serialized_string_with_unit.contains("GEOM_FORCE_TOL"));
        assert!(serialized_string_with_unit.contains("0.06"));
        assert!(serialized_string_with_unit.contains("ev/ang"));

        // 4. Test Serialization using ToCell (without unit)
        let geom_force_tol_instance_no_unit = GeomForceTol {
            value: 0.08,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&geom_force_tol_instance_no_unit.to_cell());
        assert!(
            serialized_result_no_unit.is_ok(),
            "Serialization (no unit) failed: {:?}",
            serialized_result_no_unit.err()
        );
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized GEOM_FORCE_TOL (0.08, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("GEOM_FORCE_TOL"));
        assert!(serialized_string_no_unit.contains("0.08"));
        // Check that the unit string is not present (or is the default)
    }
}
