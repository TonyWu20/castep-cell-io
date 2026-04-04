use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_f64;
use serde::{Deserialize, Serialize};
use crate::units::EnergyUnit;

/// Controls the tolerance for accepting convergence of the force constants.
///
/// Keyword type: Real
///
/// Default: Same as ELEC_ENERGY_TOL (context-dependent)
///
/// Example:
/// PHONON_ENERGY_TOL : 0.00007 eV
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "PHONON_ENERGY_TOL")]
pub struct PhononEnergyTol {
    /// The energy tolerance value.
    pub value: f64,
    /// The optional unit of the energy value.
    pub unit: Option<EnergyUnit>,
}

impl FromCellValue for PhononEnergyTol {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) => {
                let value = value_as_f64(&arr[0])?;
                let unit = if arr.len() > 1 {
                    Some(EnergyUnit::from_cell_value(&arr[1])?)
                } else {
                    None
                };
                Ok(Self { value, unit })
            }
            _ => {
                let value = value_as_f64(value)?;
                Ok(Self { value, unit: None })
            }
        }
    }
}

impl FromKeyValue for PhononEnergyTol {
    const KEY_NAME: &'static str = "PHONON_ENERGY_TOL";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for PhononEnergyTol {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PHONON_ENERGY_TOL", self.to_cell_value())
    }
}

impl ToCellValue for PhononEnergyTol {
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


