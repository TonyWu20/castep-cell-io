use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the thermostat used for a molecular dynamics calculation (NVT ensemble).
///
/// Keyword type: String
///
/// Default: MdThermostat::NoseHoover
///
/// Example:
/// MD_THERMOSTAT : Langevin
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MD_THERMOSTAT")]
pub enum MdThermostat {
    /// Nosé-Hoover thermostat
    #[serde(rename = "Nosé-Hoover", alias = "nosé-hoover", alias = "NOSÉ-HOOVER")]
    NoseHoover,
    /// Langevin thermostat
    #[serde(alias = "langevin", alias = "LANGEVIN")]
    Langevin,
}

impl Default for MdThermostat {
    fn default() -> Self {
        Self::NoseHoover // Default is Nosé-Hoover
    }
}

impl ToCell for MdThermostat {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_THERMOSTAT", self.to_cell_value())
    }
}

impl ToCellValue for MdThermostat {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                MdThermostat::NoseHoover => "Nosé-Hoover",
                MdThermostat::Langevin => "Langevin",
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
    fn test_md_thermostat_serde() {
        // Test Deserialization for various cases
        let test_cases_deser = [
            ("MD_THERMOSTAT : Nosé-Hoover", MdThermostat::NoseHoover),
            ("MD_THERMOSTAT : nosé-hoover", MdThermostat::NoseHoover),
            ("MD_THERMOSTAT : NOSÉ-HOOVER", MdThermostat::NoseHoover), // Uppercase alias
            ("MD_THERMOSTAT : Langevin", MdThermostat::Langevin),
            ("MD_THERMOSTAT : langevin", MdThermostat::Langevin),
            ("MD_THERMOSTAT : LANGEVIN", MdThermostat::Langevin), // Uppercase alias
        ];

        for (input_str, expected_thermostat) in test_cases_deser {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithMdThermostat {
                md_thermostat: MdThermostat,
            }

            let cell_file_result: Result<CellFileWithMdThermostat, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.md_thermostat, expected_thermostat,
                "Failed for input: {input_str}"
            );
        }

        // Test Serialization
        let md_thermostat_instance = MdThermostat::Langevin;
        let serialized_result = to_string(&md_thermostat_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MD_THERMOSTAT (Langevin): {serialized_string}");
        assert!(serialized_string.contains("MD_THERMOSTAT"));
        assert!(serialized_string.contains("Langevin"));

        // Test Default
        assert_eq!(MdThermostat::default(), MdThermostat::NoseHoover);
    }
}
