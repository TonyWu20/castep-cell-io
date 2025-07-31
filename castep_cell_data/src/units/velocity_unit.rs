use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the units in which velocity will be reported.
///
/// Keyword type: String
///
/// Default: VelocityUnit::AngPerPs
///
/// Example:
/// VELOCITY_UNIT : bohr/fs
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename = "VELOCITY_UNIT")]
pub enum VelocityUnit {
    /// Atomic unit of velocity
    #[serde(rename = "auv")]
    AtomicUnitOfVelocity,
    /// Å/ps
    #[serde(rename = "ang/ps")]
    AngPerPs,
    /// Å/fs
    #[serde(rename = "ang/fs")]
    AngPerFs,
    /// Bohr per picosecond
    #[serde(rename = "bohr/ps")]
    BohrPerPs,
    /// Bohr per femtosecond
    #[serde(rename = "bohr/fs")]
    BohrPerFs,
    /// Meters per second
    #[serde(rename = "m/s")]
    MetersPerSecond,
}

impl Default for VelocityUnit {
    fn default() -> Self {
        Self::AngPerPs // Default is ang/ps
    }
}

impl ToCell for VelocityUnit {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("VELOCITY_UNIT", self.to_cell_value())
    }
}

impl ToCellValue for VelocityUnit {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                VelocityUnit::AtomicUnitOfVelocity => "auv",
                VelocityUnit::AngPerPs => "ang/ps",
                VelocityUnit::AngPerFs => "ang/fs",
                VelocityUnit::BohrPerPs => "bohr/ps",
                VelocityUnit::BohrPerFs => "bohr/fs",
                VelocityUnit::MetersPerSecond => "m/s",
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
    fn test_velocity_unit_serde() {
        let test_cases = [
            ("VELOCITY_UNIT : auv", VelocityUnit::AtomicUnitOfVelocity),
            ("VELOCITY_UNIT : ang/ps", VelocityUnit::AngPerPs),
            ("VELOCITY_UNIT : bohr/fs", VelocityUnit::BohrPerFs),
            ("VELOCITY_UNIT : m/s", VelocityUnit::MetersPerSecond),
        ];

        for (input_str, expected_unit) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithVelocityUnit {
                velocity_unit: VelocityUnit,
            }

            let cell_file_result: Result<CellFileWithVelocityUnit, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.velocity_unit, expected_unit,
                "Failed for input: {}",
                input_str
            );
        }

        let velocity_unit_instance = VelocityUnit::BohrPerFs;
        let serialized_result = to_string(&velocity_unit_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized VELOCITY_UNIT (bohr/fs): {serialized_string}");
        assert!(serialized_string.contains("VELOCITY_UNIT"));
        assert!(serialized_string.contains("bohr/fs"));

        assert_eq!(VelocityUnit::default(), VelocityUnit::AngPerPs);
    }
}
