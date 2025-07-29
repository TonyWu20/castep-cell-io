use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the units in which pressure will be reported.
///
/// Keyword type: String
///
/// Default: gpa
///
/// Example:
/// PRESSURE_UNIT : atm
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename = "PRESSURE_UNIT")] // Ensures correct key name during serde
pub enum PressureUnit {
    /// Hartree per Bohr^3
    #[serde(rename = "hartree/bohr**3")]
    HartreePerBohr3,
    /// Electron Volts per Å^3
    #[serde(rename = "ev/ang**3")]
    EvPerAng3,
    /// Pascal
    #[serde(rename = "pa")]
    Pascal,
    /// Megapascal
    #[serde(rename = "mpa")]
    MegaPascal,
    /// Gigapascal (Default)
    #[serde(rename = "gpa")]
    GigaPascal,
    /// Atmosphere
    #[serde(rename = "atm")]
    Atmosphere,
    /// Bar
    #[serde(rename = "bar")]
    Bar,
    /// Megabar
    #[serde(rename = "mbar")]
    MegaBar,
}

impl PressureUnit {
    /// Returns the default unit.
    pub const fn default_unit() -> Self {
        Self::GigaPascal // Default is gpa
    }
}

impl Default for PressureUnit {
    fn default() -> Self {
        Self::default_unit()
    }
}

// Implement ToCell for PressureUnit to enable serialization via your custom backend
impl ToCell for PressureUnit {
    fn to_cell(&self) -> Cell {
        // Create a KeyValue Cell with the name "PRESSURE_UNIT" and the unit string as the value.
        Cell::KeyValue("PRESSURE_UNIT", self.to_cell_value())
    }
}

// Implement ToCellValue for PressureUnit.
impl ToCellValue for PressureUnit {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                PressureUnit::HartreePerBohr3 => "hartree/bohr**3",
                PressureUnit::EvPerAng3 => "ev/ang**3",
                PressureUnit::Pascal => "pa",
                PressureUnit::MegaPascal => "mpa",
                PressureUnit::GigaPascal => "gpa",
                PressureUnit::Atmosphere => "atm",
                PressureUnit::Bar => "bar",
                PressureUnit::MegaBar => "mbar",
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
    fn test_pressure_unit_serde() {
        // 1. Test Deserialization for each variant
        // Note: The identifiers like "hartree/bohr**3" might need special handling
        // in the parser if "**" is tokenized strangely. Assuming the parser/tokenizer
        // handles them correctly as single string tokens.
        let test_cases = [
            (
                "PRESSURE_UNIT : hartree/bohr**3",
                PressureUnit::HartreePerBohr3,
            ),
            ("PRESSURE_UNIT : ev/ang**3", PressureUnit::EvPerAng3),
            ("PRESSURE_UNIT : pa", PressureUnit::Pascal),
            ("PRESSURE_UNIT : mpa", PressureUnit::MegaPascal),
            ("PRESSURE_UNIT : gpa", PressureUnit::GigaPascal),
            ("PRESSURE_UNIT : atm", PressureUnit::Atmosphere),
            ("PRESSURE_UNIT : bar", PressureUnit::Bar),
            ("PRESSURE_UNIT : mbar", PressureUnit::MegaBar),
        ];

        for (input_str, expected_unit) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithPressureUnit {
                pressure_unit: PressureUnit,
            }

            let cell_file_result: Result<CellFileWithPressureUnit, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.pressure_unit, expected_unit,
                "Failed for input: {input_str}"
            );
        }

        // 2. Test Serialization using ToCell for one example
        let pressure_unit_instance = PressureUnit::Atmosphere;
        let serialized_result = to_string(&pressure_unit_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        println!("Serialized PRESSURE_UNIT (atm): {serialized_string}"); // Clippy suggestion
        // Basic check
        assert!(serialized_string.contains("PRESSURE_UNIT"));
        assert!(serialized_string.contains("atm"));

        // 3. Test ToCellValue for a few examples
        assert_eq!(
            PressureUnit::HartreePerBohr3.to_cell_value(),
            CellValue::String("hartree/bohr**3".to_string())
        );
        assert_eq!(
            PressureUnit::EvPerAng3.to_cell_value(),
            CellValue::String("ev/ang**3".to_string())
        );
        assert_eq!(
            PressureUnit::GigaPascal.to_cell_value(),
            CellValue::String("gpa".to_string())
        );

        // 4. Test Default
        assert_eq!(PressureUnit::default(), PressureUnit::GigaPascal);
    }
}
