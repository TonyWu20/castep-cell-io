use crate::units::EnergyUnit;
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the tolerance for accepting convergence of the free energy per atom.
///
/// Keyword type: Real
///
/// Default: 2e-5 eV per atom
///
/// Example:
/// GEOM_ENERGY_TOL : 0.00005 eV
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "GEOM_ENERGY_TOL")]
#[serde(from = "GeomEnergyTolRepr")] // Use intermediate repr for deserialization
pub struct GeomEnergyTol {
    /// The energy tolerance value.
    pub value: f64,
    /// The optional unit of the energy value.
    pub unit: Option<EnergyUnit>,
}

/// Intermediate representation for deserializing `GeomEnergyTol`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum GeomEnergyTolRepr {
    /// Format: value unit
    WithUnit(f64, EnergyUnit),
    /// Format: value (default unit eV implied)
    Essential(f64),
}

impl From<GeomEnergyTolRepr> for GeomEnergyTol {
    fn from(repr: GeomEnergyTolRepr) -> Self {
        match repr {
            GeomEnergyTolRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            GeomEnergyTolRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit (eV) implied
            },
        }
    }
}

impl ToCell for GeomEnergyTol {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("GEOM_ENERGY_TOL", self.to_cell_value())
    }
}

impl ToCellValue for GeomEnergyTol {
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
    fn test_geom_energy_tol_serde() {
        // 1. Test Deserialization with unit
        let geom_energy_tol_with_unit_str = "GEOM_ENERGY_TOL : 0.00005 ev";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithGeomEnergyTolUnit {
            geom_energy_tol: GeomEnergyTol,
        }

        let cell_file_result: Result<CellFileWithGeomEnergyTolUnit, _> =
            from_str(geom_energy_tol_with_unit_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.geom_energy_tol.value - 0.00005).abs() < 1e-10);
        assert_eq!(
            cell_file.geom_energy_tol.unit,
            Some(EnergyUnit::ElectronVolt)
        );

        // 2. Test Deserialization without unit (default unit implied)
        let geom_energy_tol_default_str = "GEOM_ENERGY_TOL : 0.00002";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithGeomEnergyTolDefault {
            geom_energy_tol: GeomEnergyTol,
        }

        let cell_file_default_result: Result<CellFileWithGeomEnergyTolDefault, _> =
            from_str(geom_energy_tol_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.geom_energy_tol.value - 0.00002).abs() < f64::EPSILON);
        assert_eq!(cell_file_default.geom_energy_tol.unit, None);

        // 3. Test Serialization using ToCell (with unit)
        let geom_energy_tol_instance_with_unit = GeomEnergyTol {
            value: 1e-5,
            unit: Some(EnergyUnit::Hartree),
        };
        let serialized_result_with_unit = to_string(&geom_energy_tol_instance_with_unit.to_cell());
        assert!(
            serialized_result_with_unit.is_ok(),
            "Serialization (with unit) failed: {:?}",
            serialized_result_with_unit.err()
        );
        let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        println!("Serialized GEOM_ENERGY_TOL (1e-5 ha): {serialized_string_with_unit}");
        assert!(serialized_string_with_unit.contains("GEOM_ENERGY_TOL"));
        assert!(
            serialized_string_with_unit.contains("1e-5")
                || serialized_string_with_unit.contains("0.00001")
        );
        assert!(serialized_string_with_unit.contains("ha"));

        // 4. Test Serialization using ToCell (without unit)
        let geom_energy_tol_instance_no_unit = GeomEnergyTol {
            value: 3e-5,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&geom_energy_tol_instance_no_unit.to_cell());
        assert!(
            serialized_result_no_unit.is_ok(),
            "Serialization (no unit) failed: {:?}",
            serialized_result_no_unit.err()
        );
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized GEOM_ENERGY_TOL (3e-5, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("GEOM_ENERGY_TOL"));
        assert!(
            serialized_string_no_unit.contains("3e-5")
                || serialized_string_no_unit.contains("0.00003")
        );
        // Check that the unit string is not present (or is the default)
    }
}
