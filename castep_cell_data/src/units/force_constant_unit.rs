use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the units in which force constants will be reported.
///
/// Keyword type: String
///
/// Default: ForceConstantUnit::EvPerAng2
///
/// Example:
/// FORCE_CONSTANT_UNIT : n/m
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename = "FORCE_CONSTANT_UNIT")]
pub enum ForceConstantUnit {
    /// Hartree per Bohr^2
    #[serde(rename = "hartree/bohr**2")]
    HartreePerBohr2,
    /// Electron Volts per Å^2
    #[serde(rename = "ev/ang**2")]
    EvPerAng2,
    /// Newton per meter
    #[serde(rename = "n/m")]
    NewtonPerMeter,
    /// Dynes per centimeter
    #[serde(rename = "dyne/cm")]
    DynesPerCentimeter,
}

impl Default for ForceConstantUnit {
    fn default() -> Self {
        Self::EvPerAng2 // Default is ev/ang**2
    }
}

impl ToCell for ForceConstantUnit {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("FORCE_CONSTANT_UNIT", self.to_cell_value())
    }
}

impl ToCellValue for ForceConstantUnit {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                ForceConstantUnit::HartreePerBohr2 => "hartree/bohr**2",
                ForceConstantUnit::EvPerAng2 => "ev/ang**2",
                ForceConstantUnit::NewtonPerMeter => "n/m",
                ForceConstantUnit::DynesPerCentimeter => "dyne/cm",
            }
            .to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_force_constant_unit_serde() {
        let test_cases = [
            (
                "FORCE_CONSTANT_UNIT : hartree/bohr**2",
                ForceConstantUnit::HartreePerBohr2,
            ),
            (
                "FORCE_CONSTANT_UNIT : ev/ang**2",
                ForceConstantUnit::EvPerAng2,
            ),
            (
                "FORCE_CONSTANT_UNIT : n/m",
                ForceConstantUnit::NewtonPerMeter,
            ),
            (
                "FORCE_CONSTANT_UNIT : dyne/cm",
                ForceConstantUnit::DynesPerCentimeter,
            ),
        ];

        for (input_str, expected_unit) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithForceConstantUnit {
                force_constant_unit: ForceConstantUnit,
            }

            let cell_file_result: Result<CellFileWithForceConstantUnit, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.force_constant_unit, expected_unit,
                "Failed for input: {}",
                input_str
            );
        }

        let force_constant_unit_instance = ForceConstantUnit::NewtonPerMeter;
        let serialized_result = to_string(&force_constant_unit_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized FORCE_CONSTANT_UNIT (n/m): {serialized_string}");
        assert!(serialized_string.contains("FORCE_CONSTANT_UNIT"));
        assert!(serialized_string.contains("n/m"));

        assert_eq!(ForceConstantUnit::default(), ForceConstantUnit::EvPerAng2);
    }
}
