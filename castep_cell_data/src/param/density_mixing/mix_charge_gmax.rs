use crate::units::InvLengthUnit;
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the maximum g-vector at which the charge density is mixed.
///
/// Keyword type: Real
///
/// Default: 1.5 Å^-1
///
/// Example:
/// MIX_CHARGE_GMAX : 0.89 1/ang
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "MIX_CHARGE_GMAX")]
#[serde(from = "MixChargeGmaxRepr")] // Use intermediate repr for deserialization
pub struct MixChargeGmax {
    /// The maximum g-vector value.
    pub value: f64,
    /// The optional unit of the inverse length value.
    pub unit: Option<InvLengthUnit>,
}

/// Intermediate representation for deserializing `MixChargeGmax`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum MixChargeGmaxRepr {
    /// Format: value unit
    WithUnit(f64, InvLengthUnit),
    /// Format: value (default unit implied)
    Essential(f64),
}

impl From<MixChargeGmaxRepr> for MixChargeGmax {
    fn from(repr: MixChargeGmaxRepr) -> Self {
        match repr {
            MixChargeGmaxRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            MixChargeGmaxRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit implied
            },
        }
    }
}

impl ToCell for MixChargeGmax {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MIX_CHARGE_GMAX", self.to_cell_value())
    }
}

impl ToCellValue for MixChargeGmax {
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
    fn test_mix_charge_gmax_serde() {
        // 1. Test Deserialization with unit
        let mix_charge_gmax_with_unit_str = "MIX_CHARGE_GMAX : 0.89 1/ang";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMixChargeGmaxUnit {
            mix_charge_gmax: MixChargeGmax,
        }

        let cell_file_result: Result<CellFileWithMixChargeGmaxUnit, _> =
            from_str(mix_charge_gmax_with_unit_str);
        // This test might fail if the parser/tokenizer has issues with "1/ang".
        // Ensure your number parser correctly handles unit boundaries.
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.mix_charge_gmax.value - 0.89).abs() < 1e-10);
        // Asserting the unit depends on how InvLengthUnit is defined.
        // assert_eq!(cell_file.mix_charge_gmax.unit, Some(InvLengthUnit::PerAngstrom));

        // 2. Test Deserialization without unit (default unit implied)
        let mix_charge_gmax_default_str = "MIX_CHARGE_GMAX : 1.5";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMixChargeGmaxDefault {
            mix_charge_gmax: MixChargeGmax,
        }

        let cell_file_default_result: Result<CellFileWithMixChargeGmaxDefault, _> =
            from_str(mix_charge_gmax_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.mix_charge_gmax.value - 1.5).abs() < f64::EPSILON);
        assert_eq!(cell_file_default.mix_charge_gmax.unit, None);

        // 3. Test Serialization using ToCell (with unit)
        let mix_charge_gmax_instance_with_unit = MixChargeGmax {
            value: 1.0,
            unit: Some(InvLengthUnit::Bohr), // Assuming this unit exists
        };
        let serialized_result_with_unit = to_string(&mix_charge_gmax_instance_with_unit.to_cell());
        assert!(
            serialized_result_with_unit.is_ok(),
            "Serialization (with unit) failed: {:?}",
            serialized_result_with_unit.err()
        );
        let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        println!("Serialized MIX_CHARGE_GMAX (1.0 1/bohr): {serialized_string_with_unit}");
        assert!(serialized_string_with_unit.contains("MIX_CHARGE_GMAX"));
        assert!(serialized_string_with_unit.contains("1.0"));
        // Check for unit string, e.g., "1/bohr"

        // 4. Test Serialization using ToCell (without unit)
        let mix_charge_gmax_instance_no_unit = MixChargeGmax {
            value: 1.2,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&mix_charge_gmax_instance_no_unit.to_cell());
        assert!(
            serialized_result_no_unit.is_ok(),
            "Serialization (no unit) failed: {:?}",
            serialized_result_no_unit.err()
        );
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized MIX_CHARGE_GMAX (1.2, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("MIX_CHARGE_GMAX"));
        assert!(serialized_string_no_unit.contains("1.2"));
        // Check that the unit string is not present
        assert!(!serialized_string_no_unit.contains("1/ang"));
        assert!(!serialized_string_no_unit.contains("1/bohr"));
    }
}
