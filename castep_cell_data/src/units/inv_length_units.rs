use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the units in which inverse length will be reported.
///
/// Keyword type: String
///
/// Default: 1/ang
///
/// Example:
/// INV_LENGTH_UNIT : 1/nm
#[derive(
    Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(rename = "INV_LENGTH_UNIT")] // Ensures correct key name during serde
pub enum InvLengthUnit {
    /// Bohr-1
    #[serde(alias = "1/BOHR", alias = "1/bohr")]
    Bohr,
    /// Meter-1
    #[serde(alias = "1/M", alias = "1/m")]
    Meter,
    /// Nanometer-1
    #[serde(alias = "1/NM", alias = "1/nm")]
    NanoMeter,
    /// Å-1
    #[serde(alias = "1/ANG", alias = "1/ang")]
    #[default]
    Angstrom,
}

// Implement ToCell for InvLengthUnit to enable serialization via your custom backend
impl ToCell for InvLengthUnit {
    fn to_cell(&self) -> Cell {
        // Create a KeyValue Cell with the name "INV_LENGTH_UNIT" and the unit string as the value.
        Cell::KeyValue("INV_LENGTH_UNIT", self.to_cell_value())
    }
}

// Implement ToCellValue for InvLengthUnit.
impl ToCellValue for InvLengthUnit {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                InvLengthUnit::Bohr => "1/bohr",
                InvLengthUnit::Meter => "1/m",
                InvLengthUnit::NanoMeter => "1/nm",
                InvLengthUnit::Angstrom => "1/ang",
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
    fn test_inv_length_unit_serde() {
        // 1. Test Deserialization for each variant
        let test_cases = [
            ("INV_LENGTH_UNIT : 1/bohr", InvLengthUnit::Bohr),
            ("INV_LENGTH_UNIT : 1/m", InvLengthUnit::Meter),
            ("INV_LENGTH_UNIT : 1/nm", InvLengthUnit::NanoMeter),
            ("INV_LENGTH_UNIT : 1/ang", InvLengthUnit::Angstrom),
        ];

        for (input_str, expected_unit) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithInvLengthUnit {
                inv_length_unit: InvLengthUnit,
            }

            let cell_file_result: Result<CellFileWithInvLengthUnit, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(cell_file.inv_length_unit, expected_unit);
        }

        // 2. Test Serialization using ToCell for one example
        let inv_length_unit_instance = InvLengthUnit::NanoMeter;
        let serialized_result = to_string(&inv_length_unit_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        println!("Serialized INV_LENGTH_UNIT (1/nm):\n{serialized_string}"); // Clippy suggestion
        // Basic check
        assert!(serialized_string.contains("INV_LENGTH_UNIT"));
        assert!(serialized_string.contains("1/nm"));

        // 3. Test ToCellValue
        assert_eq!(
            InvLengthUnit::Bohr.to_cell_value(),
            CellValue::String("1/bohr".to_string())
        );
        assert_eq!(
            InvLengthUnit::Meter.to_cell_value(),
            CellValue::String("1/m".to_string())
        );
        assert_eq!(
            InvLengthUnit::NanoMeter.to_cell_value(),
            CellValue::String("1/nm".to_string())
        );
        assert_eq!(
            InvLengthUnit::Angstrom.to_cell_value(),
            CellValue::String("1/ang".to_string())
        );

        // 4. Test Default
        assert_eq!(InvLengthUnit::default(), InvLengthUnit::Angstrom);
    }
}
