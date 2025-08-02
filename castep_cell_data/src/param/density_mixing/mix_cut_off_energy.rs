use crate::units::EnergyUnit;
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the cutoff energy for the densities used in the density mixing scheme.
///
/// Keyword type: Real
///
/// Default: The value of CUT_OFF_ENERGY
///
/// Example:
/// MIX_CUT_OFF_ENERGY : 250.0 eV
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "MIX_CUT_OFF_ENERGY")]
#[serde(from = "MixCutOffEnergyRepr")] // Use intermediate repr for deserialization
pub struct MixCutOffEnergy {
    /// The cutoff energy value.
    pub value: f64,
    /// The optional unit of the energy value.
    pub unit: Option<EnergyUnit>,
}

/// Intermediate representation for deserializing `MixCutOffEnergy`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum MixCutOffEnergyRepr {
    /// Format: value unit
    WithUnit(f64, EnergyUnit),
    /// Format: value (default unit implied or context-dependent)
    Essential(f64),
}

impl From<MixCutOffEnergyRepr> for MixCutOffEnergy {
    fn from(repr: MixCutOffEnergyRepr) -> Self {
        match repr {
            MixCutOffEnergyRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            MixCutOffEnergyRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit implied or context-dependent
            },
        }
    }
}

impl ToCell for MixCutOffEnergy {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MIX_CUT_OFF_ENERGY", self.to_cell_value())
    }
}

impl ToCellValue for MixCutOffEnergy {
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
    fn test_mix_cut_off_energy_serde() {
        // 1. Test Deserialization with unit
        let mix_cut_off_energy_with_unit_str = "MIX_CUT_OFF_ENERGY : 250.0 ev";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMixCutOffEnergyUnit {
            mix_cut_off_energy: MixCutOffEnergy,
        }

        let cell_file_result: Result<CellFileWithMixCutOffEnergyUnit, _> =
            from_str(mix_cut_off_energy_with_unit_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.mix_cut_off_energy.value - 250.0).abs() < f64::EPSILON);
        assert_eq!(
            cell_file.mix_cut_off_energy.unit,
            Some(EnergyUnit::ElectronVolt)
        );

        // 2. Test Deserialization without unit (default/unit from context)
        let mix_cut_off_energy_default_str = "MIX_CUT_OFF_ENERGY : 300.0";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMixCutOffEnergyDefault {
            mix_cut_off_energy: MixCutOffEnergy,
        }

        let cell_file_default_result: Result<CellFileWithMixCutOffEnergyDefault, _> =
            from_str(mix_cut_off_energy_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.mix_cut_off_energy.value - 300.0).abs() < f64::EPSILON);
        assert_eq!(cell_file_default.mix_cut_off_energy.unit, None);

        // 3. Test Serialization using ToCell (with unit)
        let mix_cut_off_energy_instance_with_unit = MixCutOffEnergy {
            value: 200.0,
            unit: Some(EnergyUnit::Hartree),
        };
        let serialized_result_with_unit =
            to_string(&mix_cut_off_energy_instance_with_unit.to_cell());
        assert!(
            serialized_result_with_unit.is_ok(),
            "Serialization (with unit) failed: {:?}",
            serialized_result_with_unit.err()
        );
        let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        println!("Serialized MIX_CUT_OFF_ENERGY (200.0 ha): {serialized_string_with_unit}");
        assert!(serialized_string_with_unit.contains("MIX_CUT_OFF_ENERGY"));
        assert!(serialized_string_with_unit.contains("200.0"));
        assert!(serialized_string_with_unit.contains("ha"));

        // 4. Test Serialization using ToCell (without unit)
        let mix_cut_off_energy_instance_no_unit = MixCutOffEnergy {
            value: 350.0,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&mix_cut_off_energy_instance_no_unit.to_cell());
        assert!(
            serialized_result_no_unit.is_ok(),
            "Serialization (no unit) failed: {:?}",
            serialized_result_no_unit.err()
        );
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized MIX_CUT_OFF_ENERGY (350.0, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("MIX_CUT_OFF_ENERGY"));
        assert!(serialized_string_no_unit.contains("350.0"));
        // Check that the unit string is not present
        assert!(!serialized_string_no_unit.contains("ev"));
        assert!(!serialized_string_no_unit.contains("ha"));
    }
}
