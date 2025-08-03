use castep_cell_serde::{CellValue, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the units for the electric field vector in the EXTERNAL_EFIELD block.
#[derive(
    Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(rename = "EFIELD_UNIT")] // Name for serde, though likely used via variant names
pub enum EFieldUnit {
    /// The default unit: eV/Å/electron
    #[serde(rename = "ev/ang/e", alias = "EV/ANG/E")] // CASTEP notation for eV/Å/electron
    #[default]
    EvPerAngPerE,
    /// Hartree per Bohr per electron
    #[serde(rename = "hartree/bohr/e", alias = "HARTREE/BOHR/E")]
    HartreePerBohrPerE,
    // Add other units if they become valid/used in CASTEP for this context
    #[serde(rename = "N/C", alias = "n/c")]
    NewtonPerCharge,
}

// Implement ToCellValue for EFieldUnit to allow serialization via your backend.
impl ToCellValue for EFieldUnit {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                EFieldUnit::EvPerAngPerE => "ev/ang/e",
                EFieldUnit::HartreePerBohrPerE => "hartree/bohr/e",
                EFieldUnit::NewtonPerCharge => "n/c", // Add arms for other variants
            }
            .to_string(), // Convert &str to String for CellValue::String
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCellValue, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_efield_unit_serde() {
        // Test Deserialization for each variant
        let test_cases = [
            ("EFIELD_UNIT : ev/ang/e", EFieldUnit::EvPerAngPerE),
            (
                "EFIELD_UNIT : hartree/bohr/e",
                EFieldUnit::HartreePerBohrPerE,
            ),
            ("EFIELD_UNIT : N/C", EFieldUnit::NewtonPerCharge),
        ];

        for (input_str, expected_unit) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct Wrapper {
                efield_unit: EFieldUnit,
            }

            let result: Result<Wrapper, _> = from_str(input_str);
            assert!(
                result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                result.err()
            );
            let wrapper = result.unwrap();
            assert_eq!(wrapper.efield_unit, expected_unit);
        }

        // Test Serialization/ToCellValue
        assert_eq!(
            EFieldUnit::EvPerAngPerE.to_cell_value(),
            CellValue::String("ev/ang/e".to_string())
        );
        assert_eq!(
            EFieldUnit::HartreePerBohrPerE.to_cell_value(),
            CellValue::String("hartree/bohr/e".to_string())
        );
        assert_eq!(
            EFieldUnit::NewtonPerCharge.to_cell_value(),
            CellValue::String("n/c".to_string())
        );

        // Test Default
        assert_eq!(EFieldUnit::default(), EFieldUnit::EvPerAngPerE);
    }
}
