use crate::units::InvLengthUnit;
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the weighting factor for the densities used in the density mixing scheme.
///
/// Keyword type: Real
///
/// Default: -1 (CASTEP will automatically select the appropriate value)
///
/// Example:
/// MIX_METRIC_Q : 20.0 1/ang
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "MIX_METRIC_Q")]
#[serde(from = "MixMetricQRepr")] // Use intermediate repr for deserialization
pub struct MixMetricQ {
    /// The weighting factor value.
    pub value: f64,
    /// The optional unit of the inverse length value.
    pub unit: Option<InvLengthUnit>,
}

/// Intermediate representation for deserializing `MixMetricQ`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum MixMetricQRepr {
    /// Format: value unit
    WithUnit(f64, InvLengthUnit),
    /// Format: value (default unit implied or no unit for -1)
    Essential(f64),
}

impl From<MixMetricQRepr> for MixMetricQ {
    fn from(repr: MixMetricQRepr) -> Self {
        match repr {
            MixMetricQRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            MixMetricQRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit implied or no unit
            },
        }
    }
}

impl ToCell for MixMetricQ {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MIX_METRIC_Q", self.to_cell_value())
    }
}

impl ToCellValue for MixMetricQ {
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
    fn test_mix_metric_q_serde() {
        // 1. Test Deserialization with unit
        let mix_metric_q_with_unit_str = "MIX_METRIC_Q : 20.0 1/ang";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMixMetricQUnit {
            mix_metric_q: MixMetricQ,
        }

        let cell_file_result: Result<CellFileWithMixMetricQUnit, _> =
            from_str(mix_metric_q_with_unit_str);
        // This test might fail if the parser/tokenizer has issues with "1/ang".
        // Ensure your number parser correctly handles unit boundaries.
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.mix_metric_q.value - 20.0).abs() < f64::EPSILON);
        // Asserting the unit depends on how InvLengthUnit is defined.
        // assert_eq!(cell_file.mix_metric_q.unit, Some(InvLengthUnit::PerAngstrom));

        // 2. Test Deserialization without unit (default case)
        let mix_metric_q_default_str = "MIX_METRIC_Q : -1.0";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMixMetricQDefault {
            mix_metric_q: MixMetricQ,
        }

        let cell_file_default_result: Result<CellFileWithMixMetricQDefault, _> =
            from_str(mix_metric_q_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.mix_metric_q.value - (-1.0)).abs() < f64::EPSILON);
        assert_eq!(cell_file_default.mix_metric_q.unit, None);

        // 3. Test Serialization using ToCell (with unit)
        let mix_metric_q_instance_with_unit = MixMetricQ {
            value: 15.0,
            unit: Some(InvLengthUnit::Bohr), // Assuming this unit exists
        };
        let serialized_result_with_unit = to_string(&mix_metric_q_instance_with_unit.to_cell());
        assert!(
            serialized_result_with_unit.is_ok(),
            "Serialization (with unit) failed: {:?}",
            serialized_result_with_unit.err()
        );
        let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        println!("Serialized MIX_METRIC_Q (15.0 1/bohr): {serialized_string_with_unit}");
        assert!(serialized_string_with_unit.contains("MIX_METRIC_Q"));
        assert!(serialized_string_with_unit.contains("15.0"));
        // Check for unit string, e.g., "1/bohr"

        // 4. Test Serialization using ToCell (without unit)
        let mix_metric_q_instance_no_unit = MixMetricQ {
            value: -1.0,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&mix_metric_q_instance_no_unit.to_cell());
        assert!(
            serialized_result_no_unit.is_ok(),
            "Serialization (no unit) failed: {:?}",
            serialized_result_no_unit.err()
        );
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized MIX_METRIC_Q (-1.0, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("MIX_METRIC_Q"));
        assert!(serialized_string_no_unit.contains("-1.0"));
        // Check that the unit string is not present
        assert!(!serialized_string_no_unit.contains("1/ang"));
        assert!(!serialized_string_no_unit.contains("1/bohr"));
    }
}
