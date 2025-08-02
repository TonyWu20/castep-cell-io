use crate::units::FrequencyUnit;
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Provides an estimate of the average phonon frequency at the gamma point.
///
/// Keyword type: Real
///
/// Default: 50 THz
///
/// Example:
/// GEOM_FREQUENCY_EST : 17.54 THz
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "GEOM_FREQUENCY_EST")]
#[serde(from = "GeomFrequencyEstRepr")] // Use intermediate repr for deserialization
pub struct GeomFrequencyEst {
    /// The frequency estimate value.
    pub value: f64,
    /// The optional unit of the frequency value.
    pub unit: Option<FrequencyUnit>,
}

/// Intermediate representation for deserializing `GeomFrequencyEst`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum GeomFrequencyEstRepr {
    /// Format: value unit
    WithUnit(f64, FrequencyUnit),
    /// Format: value (default unit THz implied)
    Essential(f64),
}

impl From<GeomFrequencyEstRepr> for GeomFrequencyEst {
    fn from(repr: GeomFrequencyEstRepr) -> Self {
        match repr {
            GeomFrequencyEstRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            GeomFrequencyEstRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit (THz) implied
            },
        }
    }
}

impl ToCell for GeomFrequencyEst {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("GEOM_FREQUENCY_EST", self.to_cell_value())
    }
}

impl ToCellValue for GeomFrequencyEst {
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
    fn test_geom_frequency_est_serde() {
        // 1. Test Deserialization with unit
        let geom_frequency_est_with_unit_str = "GEOM_FREQUENCY_EST : 17.54 thz";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithGeomFrequencyEstUnit {
            geom_frequency_est: GeomFrequencyEst,
        }

        let cell_file_result: Result<CellFileWithGeomFrequencyEstUnit, _> =
            from_str(geom_frequency_est_with_unit_str);
        // This test depends on FrequencyUnit correctly parsing "thz"
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.geom_frequency_est.value - 17.54).abs() < 1e-10);
        // assert_eq!(cell_file.geom_frequency_est.unit, Some(FrequencyUnit::Terahertz)); // If this variant exists

        // 2. Test Deserialization without unit (default unit implied)
        let geom_frequency_est_default_str = "GEOM_FREQUENCY_EST : 50.0";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithGeomFrequencyEstDefault {
            geom_frequency_est: GeomFrequencyEst,
        }

        let cell_file_default_result: Result<CellFileWithGeomFrequencyEstDefault, _> =
            from_str(geom_frequency_est_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.geom_frequency_est.value - 50.0).abs() < f64::EPSILON);
        assert_eq!(cell_file_default.geom_frequency_est.unit, None);

        // 3. Test Serialization using ToCell (with unit)
        // This depends on FrequencyUnit having a variant that serializes to "thz"
        let geom_frequency_est_instance_with_unit = GeomFrequencyEst {
            value: 20.0,
            unit: Some(FrequencyUnit::Terahertz), // If this variant exists
        };
        let serialized_result_with_unit =
            to_string(&geom_frequency_est_instance_with_unit.to_cell());
        assert!(
            serialized_result_with_unit.is_ok(),
            "Serialization (with unit) failed: {:?}",
            serialized_result_with_unit.err()
        );
        let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        println!("Serialized GEOM_FREQUENCY_EST (20.0 thz): {serialized_string_with_unit}");
        assert!(serialized_string_with_unit.contains("GEOM_FREQUENCY_EST"));
        assert!(serialized_string_with_unit.contains("20.0"));
        assert!(serialized_string_with_unit.contains("thz"));

        // 4. Test Serialization using ToCell (without unit)
        let geom_frequency_est_instance_no_unit = GeomFrequencyEst {
            value: 30.0,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&geom_frequency_est_instance_no_unit.to_cell());
        assert!(
            serialized_result_no_unit.is_ok(),
            "Serialization (no unit) failed: {:?}",
            serialized_result_no_unit.err()
        );
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized GEOM_FREQUENCY_EST (30.0, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("GEOM_FREQUENCY_EST"));
        assert!(serialized_string_no_unit.contains("30.0"));
        // Check that the unit string is not present (or is the default)
    }
}
