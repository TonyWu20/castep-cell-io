// File: electronic_minimisation/smearing_width.rs
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};
// Assuming EnergyUnit exists in units module
use crate::units::EnergyUnit;

/// Determines the width of the Fermi-surface smearing.
///
/// Keyword type: Real
///
/// Default: 0.2 eV
///
/// Example:
/// SMEARING_WIDTH : 0.1 eV
/// SMEARING_WIDTH : 0.1 (uses default unit, likely eV)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "SMEARING_WIDTH")]
#[serde(from = "SmearingWidthRepr")] // Use intermediate repr for deserialization
pub struct SmearingWidth {
    /// The smearing width value.
    pub value: f64,
    /// The optional unit of the energy value.
    /// If None, a default unit (likely eV) is implied.
    pub unit: Option<EnergyUnit>,
}

/// Intermediate representation for deserializing `SmearingWidth`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum SmearingWidthRepr {
    /// Format: value unit
    WithUnit(f64, EnergyUnit),
    /// Format: value (default unit implied)
    Essential(f64),
}

impl From<SmearingWidthRepr> for SmearingWidth {
    fn from(repr: SmearingWidthRepr) -> Self {
        match repr {
            SmearingWidthRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            SmearingWidthRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit implied
            },
        }
    }
}

impl ToCell for SmearingWidth {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("SMEARING_WIDTH", self.to_cell_value())
    }
}

impl ToCellValue for SmearingWidth {
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
    fn test_smearing_width_serde() {
        // 1. Test Deserialization with unit
        let smearing_width_with_unit_str = "SMEARING_WIDTH : 0.1 ev";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithSmearingWidthUnit {
            smearing_width: SmearingWidth,
        }

        let cell_file_result: Result<CellFileWithSmearingWidthUnit, _> =
            from_str(smearing_width_with_unit_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.smearing_width.value - 0.1).abs() < 1e-10);
        assert_eq!(
            cell_file.smearing_width.unit,
            Some(EnergyUnit::ElectronVolt)
        );

        // 2. Test Deserialization without unit (default unit implied)
        let smearing_width_default_str = "SMEARING_WIDTH : 0.2";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithSmearingWidthDefault {
            smearing_width: SmearingWidth,
        }

        let cell_file_default_result: Result<CellFileWithSmearingWidthDefault, _> =
            from_str(smearing_width_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.smearing_width.value - 0.2).abs() < 1e-10);
        assert_eq!(cell_file_default.smearing_width.unit, None);

        // 3. Test Serialization using ToCell (with unit)
        let smearing_width_instance_with_unit = SmearingWidth {
            value: 0.15,
            unit: Some(EnergyUnit::Hartree),
        };
        let serialized_result_with_unit = to_string(&smearing_width_instance_with_unit.to_cell());
        assert!(
            serialized_result_with_unit.is_ok(),
            "Serialization (with unit) failed: {:?}",
            serialized_result_with_unit.err()
        );
        let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        println!("Serialized SMEARING_WIDTH (0.15 ha):\n{serialized_string_with_unit}");
        assert!(serialized_string_with_unit.contains("SMEARING_WIDTH"));
        assert!(serialized_string_with_unit.contains("0.15"));
        assert!(serialized_string_with_unit.contains("ha"));

        // 4. Test Serialization using ToCell (without unit)
        let smearing_width_instance_no_unit = SmearingWidth {
            value: 0.25,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&smearing_width_instance_no_unit.to_cell());
        assert!(
            serialized_result_no_unit.is_ok(),
            "Serialization (no unit) failed: {:?}",
            serialized_result_no_unit.err()
        );
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized SMEARING_WIDTH (0.25, no unit):\n{serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("SMEARING_WIDTH"));
        assert!(serialized_string_no_unit.contains("0.25"));
        // Check that the unit string is not present
        // Note: This check might be brittle depending on exact float formatting
        // A more robust check would parse the serialized string.
        // For now, we assume the absence of a known unit string indicates success.
        assert!(!serialized_string_no_unit.contains("ev"));
        assert!(!serialized_string_no_unit.contains("ha"));
        // ... (add checks for other unit strings if necessary)
    }
}
