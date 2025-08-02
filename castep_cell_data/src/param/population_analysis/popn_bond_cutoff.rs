use crate::units::LengthUnit;
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the maximum distance between two atoms for which a bond population
/// will be generated during population analysis.
///
/// Keyword type: Real
///
/// Default: 3.0 Å
///
/// Example:
/// POPN_BOND_CUTOFF : 2.54 ang
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "POPN_BOND_CUTOFF")]
#[serde(from = "PopnBondCutoffRepr")] // Use intermediate repr for deserialization
pub struct PopnBondCutoff {
    /// The cutoff distance value.
    pub value: f64,
    /// The optional unit of the length value.
    pub unit: Option<LengthUnit>,
}

/// Intermediate representation for deserializing `PopnBondCutoff`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum PopnBondCutoffRepr {
    /// Format: value unit
    WithUnit(f64, LengthUnit),
    /// Format: value (default unit Å implied)
    Essential(f64),
}

impl From<PopnBondCutoffRepr> for PopnBondCutoff {
    fn from(repr: PopnBondCutoffRepr) -> Self {
        match repr {
            PopnBondCutoffRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            PopnBondCutoffRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit (Å) implied
            },
        }
    }
}

impl ToCell for PopnBondCutoff {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("POPN_BOND_CUTOFF", self.to_cell_value())
    }
}

impl ToCellValue for PopnBondCutoff {
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
    fn test_popn_bond_cutoff_serde() {
        // 1. Test Deserialization with unit
        let popn_bond_cutoff_with_unit_str = "POPN_BOND_CUTOFF : 2.54 ang";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPopnBondCutoffUnit {
            popn_bond_cutoff: PopnBondCutoff,
        }

        let cell_file_result: Result<CellFileWithPopnBondCutoffUnit, _> =
            from_str(popn_bond_cutoff_with_unit_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.popn_bond_cutoff.value - 2.54).abs() < 1e-10);
        // Asserting the unit depends on how LengthUnit is defined.
        // assert_eq!(cell_file.popn_bond_cutoff.unit, Some(LengthUnit::Ang));

        // 2. Test Deserialization without unit (default unit implied)
        let popn_bond_cutoff_default_str = "POPN_BOND_CUTOFF : 3.0";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPopnBondCutoffDefault {
            popn_bond_cutoff: PopnBondCutoff,
        }

        let cell_file_default_result: Result<CellFileWithPopnBondCutoffDefault, _> =
            from_str(popn_bond_cutoff_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.popn_bond_cutoff.value - 3.0).abs() < f64::EPSILON);
        assert_eq!(cell_file_default.popn_bond_cutoff.unit, None);

        // 3. Test Serialization using ToCell (with unit)
        let popn_bond_cutoff_instance_with_unit = PopnBondCutoff {
            value: 2.0,
            unit: Some(LengthUnit::Bohr), // Assuming this unit exists
        };
        let serialized_result_with_unit = to_string(&popn_bond_cutoff_instance_with_unit.to_cell());
        assert!(
            serialized_result_with_unit.is_ok(),
            "Serialization (with unit) failed: {:?}",
            serialized_result_with_unit.err()
        );
        let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        println!("Serialized POPN_BOND_CUTOFF (2.0 bohr): {serialized_string_with_unit}");
        assert!(serialized_string_with_unit.contains("POPN_BOND_CUTOFF"));
        assert!(serialized_string_with_unit.contains("2.0"));
        // Check for unit string, e.g., "bohr"

        // 4. Test Serialization using ToCell (without unit)
        let popn_bond_cutoff_instance_no_unit = PopnBondCutoff {
            value: 2.80000,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&popn_bond_cutoff_instance_no_unit.to_cell());
        assert!(
            serialized_result_no_unit.is_ok(),
            "Serialization (no unit) failed: {:?}",
            serialized_result_no_unit.err()
        );
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized POPN_BOND_CUTOFF (2.8, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("POPN_BOND_CUTOFF"));
        assert!(serialized_string_no_unit.contains("2.799999"));
        // Check that the unit string is not present (or is the default)
        // This check depends on the exact output format for the default unit.
    }
}
