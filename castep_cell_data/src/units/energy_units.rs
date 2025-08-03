use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the units in which energies will be reported.
///
/// Keyword type: String
///
/// Default: EnergyUnit::Ev
///
/// Example:
/// ENERGY_UNIT : kcal/mol
#[derive(
    Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(rename = "ENERGY_UNIT")]
pub enum EnergyUnit {
    /// Hartree
    #[serde(alias = "HA", alias = "ha")]
    Hartree,
    /// Millihartree
    #[serde(alias = "MHA", alias = "mha")]
    Millihartree,
    /// Electron Volt
    #[serde(alias = "EV", alias = "ev")]
    #[default]
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
    Wavenumber,
    /// Kelvin
    #[serde(alias = "K", alias = "k")]
    Kelvin,
}

impl ToCell for EnergyUnit {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("ENERGY_UNIT", self.to_cell_value())
    }
}

impl ToCellValue for EnergyUnit {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                EnergyUnit::Hartree => "ha",
                EnergyUnit::Millihartree => "mha",
                EnergyUnit::ElectronVolt => "ev",
                EnergyUnit::MilliElectronVolt => "mev",
                EnergyUnit::Rydberg => "ry",
                EnergyUnit::Millirydberg => "mry",
                EnergyUnit::KilojoulesPerMole => "kj/mol",
                EnergyUnit::KilocaloriesPerMole => "kcal/mol",
                EnergyUnit::Joules => "j",
                EnergyUnit::Erg => "erg",
                EnergyUnit::Hertz => "hz",
                EnergyUnit::Megahertz => "mhz",
                EnergyUnit::Gigahertz => "ghz",
                EnergyUnit::Terahertz => "thz",
                EnergyUnit::Wavenumber => "cm-1",
                EnergyUnit::Kelvin => "k",
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
    fn test_energy_unit_serde() {
        let test_cases = [
            ("ENERGY_UNIT : ha", EnergyUnit::Hartree),
            ("ENERGY_UNIT : ev", EnergyUnit::ElectronVolt),
            ("ENERGY_UNIT : kcal/mol", EnergyUnit::KilocaloriesPerMole),
            ("ENERGY_UNIT : cm-1", EnergyUnit::Wavenumber),
        ];

        for (input_str, expected_unit) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithEnergyUnit {
                energy_unit: EnergyUnit,
            }

            let cell_file_result: Result<CellFileWithEnergyUnit, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.energy_unit, expected_unit,
                "Failed for input: {input_str}"
            );
        }

        let energy_unit_instance = EnergyUnit::KilocaloriesPerMole;
        let serialized_result = to_string(&energy_unit_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized ENERGY_UNIT (kcal/mol): {serialized_string}");
        assert!(serialized_string.contains("ENERGY_UNIT"));
        assert!(serialized_string.contains("kcal/mol"));

        assert_eq!(EnergyUnit::default(), EnergyUnit::ElectronVolt);
    }
}
