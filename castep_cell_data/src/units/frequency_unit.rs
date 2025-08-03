use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the units in which frequency will be reported.
///
/// Keyword type: String
///
/// Default: FrequencyUnit::Wavenumber
///
/// Example:
/// FREQUENCY_UNIT : hz
#[derive(
    Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(rename = "FREQUENCY_UNIT")]
pub enum FrequencyUnit {
    /// Hartree
    #[serde(alias = "HA", alias = "ha")]
    Hartree,
    /// Millihartree
    #[serde(alias = "MHA", alias = "mha")]
    Millihartree,
    /// Electron Volt
    #[serde(alias = "EV", alias = "ev")]
    ElectronVolt,
    /// Milli-electron Volt
    #[serde(alias = "MEV", alias = "mev")]
    MilliElectronVolt,
    /// Rydberg
    #[serde(alias = "RY", alias = "ry")]
    Rydberg,
    /// Millirydberg
    #[serde(alias = "MRY", alias = "mry")]
    Millirydberg,
    /// Kilojoules per mole
    #[serde(alias = "KJ/MOL", alias = "kj/mol")]
    KilojoulesPerMole,
    /// Kilocalories per mole
    #[serde(alias = "KCAL/MOL", alias = "kcal/mol")]
    KilocaloriesPerMole,
    /// Joules
    #[serde(alias = "J", alias = "j")]
    Joules,
    /// Erg
    #[serde(alias = "ERG", alias = "erg")]
    Erg,
    /// Hertz
    #[serde(alias = "HZ", alias = "hz")]
    Hertz,
    /// Megahertz
    #[serde(alias = "MHZ", alias = "mhz")]
    Megahertz,
    /// Gigahertz
    #[serde(alias = "GHZ", alias = "ghz")]
    Gigahertz,
    /// Terahertz
    #[serde(alias = "THZ", alias = "thz")]
    Terahertz,
    /// Wavenumber
    #[serde(alias = "CM-1", alias = "cm-1")]
    #[default]
    Wavenumber,
    /// Kelvin
    #[serde(alias = "K", alias = "k")]
    Kelvin,
}

impl ToCell for FrequencyUnit {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("FREQUENCY_UNIT", self.to_cell_value())
    }
}

impl ToCellValue for FrequencyUnit {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                FrequencyUnit::Hartree => "ha",
                FrequencyUnit::Millihartree => "mha",
                FrequencyUnit::ElectronVolt => "ev",
                FrequencyUnit::MilliElectronVolt => "mev",
                FrequencyUnit::Rydberg => "ry",
                FrequencyUnit::Millirydberg => "mry",
                FrequencyUnit::KilojoulesPerMole => "kj/mol",
                FrequencyUnit::KilocaloriesPerMole => "kcal/mol",
                FrequencyUnit::Joules => "j",
                FrequencyUnit::Erg => "erg",
                FrequencyUnit::Hertz => "hz",
                FrequencyUnit::Megahertz => "mhz",
                FrequencyUnit::Gigahertz => "ghz",
                FrequencyUnit::Terahertz => "thz",
                FrequencyUnit::Wavenumber => "cm-1",
                FrequencyUnit::Kelvin => "k",
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
    fn test_frequency_unit_serde() {
        let test_cases = [
            ("FREQUENCY_UNIT : ha", FrequencyUnit::Hartree),
            ("FREQUENCY_UNIT : ev", FrequencyUnit::ElectronVolt),
            ("FREQUENCY_UNIT : hz", FrequencyUnit::Hertz),
            ("FREQUENCY_UNIT : cm-1", FrequencyUnit::Wavenumber),
            ("FREQUENCY_UNIT : k", FrequencyUnit::Kelvin),
        ];

        for (input_str, expected_unit) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithFrequencyUnit {
                frequency_unit: FrequencyUnit,
            }

            let cell_file_result: Result<CellFileWithFrequencyUnit, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.frequency_unit, expected_unit,
                "Failed for input: {input_str}"
            );
        }

        let frequency_unit_instance = FrequencyUnit::Hertz;
        let serialized_result = to_string(&frequency_unit_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized FREQUENCY_UNIT (hz): {serialized_string}");
        assert!(serialized_string.contains("FREQUENCY_UNIT"));
        assert!(serialized_string.contains("hz"));

        assert_eq!(FrequencyUnit::default(), FrequencyUnit::Wavenumber);
    }
}
