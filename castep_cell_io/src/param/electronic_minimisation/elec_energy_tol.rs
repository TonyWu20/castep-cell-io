use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromKeyValue, CResult};
use castep_cell_fmt::query::*;
use castep_cell_fmt::parse::FromCellValue;
use crate::units::EnergyUnit;

/// Controls the tolerance for accepting convergence of the total energy in an electronic minimization.
///
/// Keyword type: Real
///
/// Default: 1e-5 eV per atom
///
/// Example:
/// ELEC_ENERGY_TOL : 0.00007 eV
/// ELEC_ENERGY_TOL : 0.00007 (uses default unit, likely eV)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ElecEnergyTol {
    /// The energy tolerance value.
    pub value: f64,
    /// The optional unit of the energy value.
    /// If None, a default unit (likely eV) is implied.
    pub unit: Option<EnergyUnit>,
}

impl FromKeyValue for ElecEnergyTol {
    const KEY_NAME: &'static str = "ELEC_ENERGY_TOL";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if !arr.is_empty() => {
                let val = value_as_f64(&arr[0])?;
                let unit = if arr.len() > 1 {
                    Some(EnergyUnit::from_cell_value(&arr[1])?)
                } else {
                    None
                };
                Ok(Self { value: val, unit })
            }
            _ => {
                let val = value_as_f64(value)?;
                Ok(Self { value: val, unit: None })
            }
        }
    }
}

impl ToCell for ElecEnergyTol {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("ELEC_ENERGY_TOL", self.to_cell_value())
    }
}

impl ToCellValue for ElecEnergyTol {
    fn to_cell_value(&self) -> CellValue<'_> {
        // Create a CellValue::Array containing the value and optionally the unit
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


