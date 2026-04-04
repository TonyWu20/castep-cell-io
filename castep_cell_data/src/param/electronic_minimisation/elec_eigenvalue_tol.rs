use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromKeyValue, CResult};
use castep_cell_io::query::*;
use castep_cell_io::parse::FromCellValue;
use crate::units::EnergyUnit;

/// Controls the tolerance for accepting convergence of a single eigenvalue during density mixing minimization.
///
/// Keyword type: Real
///
/// Default: The lower of 1e-6 eV and ELEC_ENERGY_TOL*NATOMS/NBANDS
///
/// Example:
/// ELEC_EIGENVALUE_TOL : 0.000007 eV
/// ELEC_EIGENVALUE_TOL : 0.000007 (uses default unit, likely eV)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ElecEigenvalueTol {
    /// The eigenvalue tolerance value.
    pub value: f64,
    /// The optional unit of the energy value.
    /// If None, a default unit (likely eV) is implied.
    pub unit: Option<EnergyUnit>,
}

impl FromKeyValue for ElecEigenvalueTol {
    const KEY_NAME: &'static str = "ELEC_EIGENVALUE_TOL";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() >= 1 => {
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

impl ToCell for ElecEigenvalueTol {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("ELEC_EIGENVALUE_TOL", self.to_cell_value())
    }
}

impl ToCellValue for ElecEigenvalueTol {
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


