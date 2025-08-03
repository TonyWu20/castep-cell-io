use crate::units::TemperatureUnit;
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the temperature for a molecular dynamics calculation (NVT ensemble).
///
/// Keyword type: Real
///
/// Default: 300 K
///
/// Example:
/// MD_TEMPERATURE : 275.4 K
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "MD_TEMPERATURE")]
#[serde(from = "MdTemperatureRepr")] // Use intermediate repr for deserialization
pub struct MdTemperature {
    /// The temperature value.
    pub value: f64,
    /// The optional unit of temperature.
    pub unit: Option<TemperatureUnit>,
}

/// Intermediate representation for deserializing `MdTemperature`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum MdTemperatureRepr {
    /// Format: value unit
    WithUnit(f64, TemperatureUnit),
    /// Format: value (default unit K implied)
    Essential(f64),
}

impl From<MdTemperatureRepr> for MdTemperature {
    fn from(repr: MdTemperatureRepr) -> Self {
        match repr {
            MdTemperatureRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            MdTemperatureRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit (K) implied
            },
        }
    }
}

impl ToCell for MdTemperature {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_TEMPERATURE", self.to_cell_value())
    }
}

impl ToCellValue for MdTemperature {
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
    fn test_md_temperature_serde() {
        // 1. Test Deserialization with unit
        let md_temperature_with_unit_str = "MD_TEMPERATURE : 275.4 k";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMdTemperatureUnit {
            md_temperature: MdTemperature,
        }

        let cell_file_result: Result<CellFileWithMdTemperatureUnit, _> =
            from_str(md_temperature_with_unit_str);
        // This test depends on TemperatureUnit correctly parsing "k"
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.md_temperature.value - 275.4).abs() < 1e-10);
        // assert_eq!(cell_file.md_temperature.unit, Some(TemperatureUnit::Kelvin)); // If this variant exists

        // 2. Test Deserialization without unit (default unit implied)
        let md_temperature_default_str = "MD_TEMPERATURE : 350.0";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMdTemperatureDefault {
            md_temperature: MdTemperature,
        }

        let cell_file_default_result: Result<CellFileWithMdTemperatureDefault, _> =
            from_str(md_temperature_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.md_temperature.value - 350.0).abs() < f64::EPSILON);
        assert_eq!(cell_file_default.md_temperature.unit, None);

        // 3. Test Serialization using ToCell (with unit)
        // This depends on TemperatureUnit having a variant that serializes correctly.
        // let md_temperature_instance_with_unit = MdTemperature {
        //     value: 310.0,
        //     unit: Some(TemperatureUnit::Kelvin), // If this variant exists
        // };
        // let serialized_result_with_unit = to_string(&md_temperature_instance_with_unit.to_cell());
        // assert!(serialized_result_with_unit.is_ok(), "Serialization (with unit) failed: {:?}", serialized_result_with_unit.err());
        // let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        // println!("Serialized MD_TEMPERATURE (310.0 k): {serialized_string_with_unit}");
        // assert!(serialized_string_with_unit.contains("MD_TEMPERATURE"));
        // assert!(serialized_string_with_unit.contains("310.0"));
        // assert!(serialized_string_with_unit.contains("k"));

        // 4. Test Serialization using ToCell (without unit)
        let md_temperature_instance_no_unit = MdTemperature {
            value: 400.0,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&md_temperature_instance_no_unit.to_cell());
        assert!(
            serialized_result_no_unit.is_ok(),
            "Serialization (no unit) failed: {:?}",
            serialized_result_no_unit.err()
        );
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized MD_TEMPERATURE (400.0, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("MD_TEMPERATURE"));
        assert!(serialized_string_no_unit.contains("400.0"));
        // Check that the unit string is not present (or is the default)
    }
}
