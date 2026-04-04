use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_f64;
use serde::{Deserialize, Serialize};
// Assuming VolumeUnit exists in units module
use crate::units::VolumeUnit;

/// Controls the tolerance for accepting convergence of the field constants.
///
/// Keyword type: Real
///
/// Default: 1e-5 Å^3
///
/// Example:
/// EFIELD_ENERGY_TOL : 0.000002 ANG**3
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "EFIELD_ENERGY_TOL")]
#[serde(from = "EfieldEnergyTolRepr")] // Use intermediate repr for deserialization
pub struct EfieldEnergyTol {
    /// The energy tolerance value.
    pub value: f64,
    /// The optional unit of the volume value.
    pub unit: Option<VolumeUnit>,
}

/// Intermediate representation for deserializing `EfieldEnergyTol`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum EfieldEnergyTolRepr {
    /// Format: value unit
    WithUnit(f64, VolumeUnit),
    /// Format: value (default unit Angstrom^3 implied)
    Essential(f64),
}

impl From<EfieldEnergyTolRepr> for EfieldEnergyTol {
    fn from(repr: EfieldEnergyTolRepr) -> Self {
        match repr {
            EfieldEnergyTolRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            EfieldEnergyTolRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit (Angstrom^3) implied
            },
        }
    }
}

impl FromCellValue for EfieldEnergyTol {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) => {
                let v = value_as_f64(&arr[0])?;
                let unit = arr.get(1).and_then(|u| VolumeUnit::from_cell_value(u).ok());
                Ok(Self { value: v, unit })
            }
            _ => Ok(Self {
                value: value_as_f64(value)?,
                unit: None,
            }),
        }
    }
}

impl FromKeyValue for EfieldEnergyTol {
    const KEY_NAME: &'static str = "EFIELD_ENERGY_TOL";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for EfieldEnergyTol {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("EFIELD_ENERGY_TOL", self.to_cell_value())
    }
}

impl ToCellValue for EfieldEnergyTol {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(
            [
                CellValue::Float(self.value),
                self.unit
                    .as_ref()
                    .map(|u| u.to_cell_value())
                    .unwrap_or(CellValue::Null),
            ]
            .to_vec(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value_float_only() {
        let val = CellValue::Float(0.000002);
        let result = EfieldEnergyTol::from_cell_value(&val).unwrap();
        assert_eq!(result.value, 0.000002);
        assert!(result.unit.is_none());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(EfieldEnergyTol::KEY_NAME, "EFIELD_ENERGY_TOL");
    }
}

