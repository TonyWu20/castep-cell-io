use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the units in which volume will be reported.
///
/// Keyword type: String
///
/// Default: VolumeUnit::Ang3
///
/// Example:
/// VOLUME_UNIT : nm**3
#[derive(
    Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(rename = "VOLUME_UNIT")]
pub enum VolumeUnit {
    /// Bohr^3
    #[serde(alias = "BOHR**3", alias = "bohr**3")]
    Bohr3,
    /// Meter^3
    #[serde(alias = "M**3", alias = "m**3")]
    Meter3,
    /// Centimeter^3
    #[serde(alias = "CM**3", alias = "cm**3")]
    Centimeter3,
    /// Nanometer^3
    #[serde(alias = "NM**3", alias = "nm**3")]
    Nanometer3,
    /// Ångstrom^3
    #[serde(alias = "ANG**3", alias = "ang**3")]
    #[default]
    Ang3,
}

impl ToCell for VolumeUnit {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("VOLUME_UNIT", self.to_cell_value())
    }
}

impl ToCellValue for VolumeUnit {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                VolumeUnit::Bohr3 => "bohr**3",
                VolumeUnit::Meter3 => "m**3",
                VolumeUnit::Centimeter3 => "cm**3",
                VolumeUnit::Nanometer3 => "nm**3",
                VolumeUnit::Ang3 => "ang**3",
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
    fn test_volume_unit_serde() {
        let test_cases = [
            ("VOLUME_UNIT : bohr**3", VolumeUnit::Bohr3),
            ("VOLUME_UNIT : m**3", VolumeUnit::Meter3),
            ("VOLUME_UNIT : nm**3", VolumeUnit::Nanometer3),
            ("VOLUME_UNIT : ang**3", VolumeUnit::Ang3),
        ];

        for (input_str, expected_unit) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithVolumeUnit {
                volume_unit: VolumeUnit,
            }

            let cell_file_result: Result<CellFileWithVolumeUnit, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.volume_unit, expected_unit,
                "Failed for input: {input_str}"
            );
        }

        let volume_unit_instance = VolumeUnit::Nanometer3;
        let serialized_result = to_string(&volume_unit_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized VOLUME_UNIT (nm**3): {serialized_string}");
        assert!(serialized_string.contains("VOLUME_UNIT"));
        assert!(serialized_string.contains("nm**3"));

        assert_eq!(VolumeUnit::default(), VolumeUnit::Ang3);
    }
}
