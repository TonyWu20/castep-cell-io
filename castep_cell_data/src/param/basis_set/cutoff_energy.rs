use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};
// Assuming EnergyUnit exists in units module
use crate::units::EnergyUnit;

/// Specifies the cutoff energy for the plane wave basis sets.
///
/// Keyword type: Real
///
/// Default: The value associated with the FINE level of accuracy for the pseudopotentials.
///
/// Example:
/// CUT_OFF_ENERGY : 125 eV
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "CUT_OFF_ENERGY")]
#[serde(from = "CutOffEnergyRepr")]
pub struct CutOffEnergy {
    /// The cutoff energy value.
    pub value: f64,
    /// The unit of the energy value.
    pub unit: Option<EnergyUnit>,
}

// Intermediate representation for deserialization
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum CutOffEnergyRepr {
    ValueOnly(f64),
    WithUnit(f64, EnergyUnit),
}

impl From<CutOffEnergyRepr> for CutOffEnergy {
    fn from(value: CutOffEnergyRepr) -> Self {
        match value {
            CutOffEnergyRepr::ValueOnly(value) => Self { value, unit: None },
            CutOffEnergyRepr::WithUnit(value, energy_unit) => Self {
                value,
                unit: Some(energy_unit),
            },
        }
    }
}

impl ToCell for CutOffEnergy {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue(
            "CUT_OFF_ENERGY",
            CellValue::Array(vec![
                CellValue::Float(self.value),
                self.unit
                    .as_ref()
                    .map(|u| u.to_cell_value())
                    .unwrap_or(CellValue::Null),
            ]),
        )
    }
}

impl ToCellValue for CutOffEnergy {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(vec![
            CellValue::Float(self.value),
            self.unit
                .as_ref()
                .map(|u| u.to_cell_value())
                .unwrap_or(CellValue::Null),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_cut_off_energy_serde() {
        let cut_off_energy_str_with_unit = "CUT_OFF_ENERGY : 125.000000 ev";
        let cut_off_energy_str = "CUT_OFF_ENERGY : 125.000000";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithCutOffEnergy {
            cut_off_energy: CutOffEnergy,
        }

        let cell_file_result: Result<CellFileWithCutOffEnergy, _> =
            from_str(cut_off_energy_str_with_unit);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.cut_off_energy.value - 125.0).abs() < f64::EPSILON);
        assert_eq!(
            cell_file.cut_off_energy.unit,
            Some(EnergyUnit::ElectronVolt)
        );
        let cell_file_result: Result<CellFileWithCutOffEnergy, _> = from_str(cut_off_energy_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (without unit) failed: {:?}",
            cell_file_result.err()
        );

        let cut_off_energy_instance = CutOffEnergy {
            value: 200.0,
            unit: Some(EnergyUnit::Hartree),
        };
        let serialized_result = to_string(&cut_off_energy_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized CUT_OFF_ENERGY (200.0 ha): {serialized_string}");
        assert!(serialized_string.contains("CUT_OFF_ENERGY"));
        assert!(serialized_string.contains("200.0"));
        assert!(serialized_string.contains("ha"));
        let cut_off_energy_instance = CutOffEnergy {
            value: 380.0,
            unit: None,
        };
        let serialized_result = to_string(&cut_off_energy_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized CUT_OFF_ENERGY (380.0): {serialized_string}");
        assert!(serialized_string.contains("CUT_OFF_ENERGY"));
        assert!(serialized_string.contains("380.0"));
    }
}
