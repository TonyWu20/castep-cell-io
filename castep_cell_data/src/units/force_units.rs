// File: force_unit.rs (or part of your main module structure)

use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the units in which force will be reported.
///
/// Keyword type: String
///
/// Default: ev/ang
///
/// Example:
/// FORCE_UNIT : n
#[derive(
    Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(rename = "FORCE_UNIT")] // Ensures correct key name during serde
pub enum ForceUnit {
    /// Hartree per Bohr
    #[serde(rename = "hartree/bohr")]
    HartreePerBohr,
    /// Electron volts per Ångström
    #[serde(rename = "ev/ang")]
    #[default]
    EvPerAng,
    /// Newton
    #[serde(rename = "n")]
    Newton,
}

// Implement ToCell for ForceUnit to enable serialization via your custom backend
// Note: ForceUnit itself is an enum, not a struct holding a value+unit like SymmetryTol.
// It represents the unit keyword value directly.
// So, ToCell would be used if FORCE_UNIT were a top-level item to serialize on its own,
// though typically it's serialized as part of a larger structure (like SymmetryTol.to_cell_value()).
impl ToCell for ForceUnit {
    fn to_cell(&self) -> Cell {
        // Create a KeyValue Cell with the name "FORCE_UNIT" and the unit string as the value.
        Cell::KeyValue("FORCE_UNIT", self.to_cell_value())
    }
}

// Implement ToCellValue for ForceUnit.
// This is the primary way it will be serialized when used as a value (e.g., in SymmetryTol).
impl ToCellValue for ForceUnit {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                ForceUnit::HartreePerBohr => "hartree/bohr",
                ForceUnit::EvPerAng => "ev/ang",
                ForceUnit::Newton => "n",
            }
            .to_string(), // Convert &str to String for CellValue::String
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_force_unit_serde() {
        // 1. Test Deserialization for each variant
        let test_cases = [
            ("FORCE_UNIT : hartree/bohr", ForceUnit::HartreePerBohr),
            ("FORCE_UNIT : ev/ang", ForceUnit::EvPerAng),
            ("FORCE_UNIT : n", ForceUnit::Newton),
        ];

        for (input_str, expected_unit) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithForceUnit {
                force_unit: ForceUnit,
            }

            let cell_file_result: Result<CellFileWithForceUnit, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(cell_file.force_unit, expected_unit);
        }

        // 2. Test Serialization using ToCell for one example
        let force_unit_instance = ForceUnit::Newton;
        let serialized_result = to_string(&force_unit_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        println!("Serialized FORCE_UNIT (Newton):\n{serialized_string}"); // Clippy suggestion
        // Basic check
        assert!(serialized_string.contains("FORCE_UNIT"));
        assert!(serialized_string.contains("n"));

        // 3. Test ToCellValue
        assert_eq!(
            ForceUnit::HartreePerBohr.to_cell_value(),
            CellValue::String("hartree/bohr".to_string())
        );
        assert_eq!(
            ForceUnit::EvPerAng.to_cell_value(),
            CellValue::String("ev/ang".to_string())
        );
        assert_eq!(
            ForceUnit::Newton.to_cell_value(),
            CellValue::String("n".to_string())
        );

        // 4. Test Default
        assert_eq!(ForceUnit::default(), ForceUnit::EvPerAng);
    }
}
