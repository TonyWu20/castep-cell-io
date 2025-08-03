use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Represents the unit of mass.
///
/// Keyword type: String
///
/// Default: amu
///
/// Example: MASS_UNIT : kg
#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default, // Derive Default
)]
// #[serde(rename = "MASS_UNIT")] // Ensures correct key name during serde if used directly
pub enum MassUnit {
    /// Electron mass
    #[serde(alias = "ME", alias = "me")]
    ElectronMass,
    /// Atomic mass unit
    #[default] // Specifies the default variant
    #[serde(alias = "AMU", alias = "amu")]
    AtomicMassUnit,
    /// Kilogram
    #[serde(alias = "KG", alias = "kg")]
    Kilogram,
    /// Gram
    #[serde(alias = "G", alias = "g")]
    Gram,
}

impl ToCellValue for MassUnit {
    /// Converts the enum variant into the corresponding `CellValue::String`.
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                MassUnit::ElectronMass => "me",
                MassUnit::AtomicMassUnit => "amu",
                MassUnit::Kilogram => "kg",
                MassUnit::Gram => "g",
            }
            .to_string(), // Convert &str to String
        )
    }
}

impl ToCell for MassUnit {
    fn to_cell(&self) -> castep_cell_serde::Cell {
        Cell::KeyValue("MASS_UNIT", self.to_cell_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCellValue, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_mass_unit_serde() {
        // 1. Test Deserialization for all variants
        let test_cases = [
            ("me", MassUnit::ElectronMass),
            ("amu", MassUnit::AtomicMassUnit),
            ("kg", MassUnit::Kilogram),
            ("g", MassUnit::Gram),
        ];

        for (input_str, expected_unit) in test_cases {
            let mass_unit_str = format!("MASS_UNIT : {input_str}");
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithMassUnit {
                mass_unit: MassUnit,
            }

            let cell_file_result: Result<CellFileWithMassUnit, _> = from_str(&mass_unit_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(cell_file.mass_unit, expected_unit);
            println!("{}", to_string(&cell_file.mass_unit.to_cell()).unwrap());
        }

        // 2. Test Default
        assert_eq!(MassUnit::default(), MassUnit::AtomicMassUnit);

        // 3. Test Serialization using ToCellValue
        let mass_unit_instance = MassUnit::Kilogram;
        let serialized_result = to_string(&mass_unit_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        println!("Serialized MASS_UNIT (kg): {serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("kg")); // Direct string comparison for units is usually fine
    }
}
