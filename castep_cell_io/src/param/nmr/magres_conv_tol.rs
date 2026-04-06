use crate::units::EnergyUnit;
use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::{value_as_f64, row_as_f64_n};

/// Controls the tolerance for accepting convergence of the first-order perturbed wavefunctions.
///
/// Keyword type: Real
///
/// Default:
/// 1e-12 when MAGRES_METHOD : crystal
/// 1e-9 when MAGRES_METHOD : molecular
///
/// Example:
/// MAGRES_CONV_TOL = 0.00007 eV
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MagresConvTol {
    /// The convergence tolerance value.
    pub value: f64,
    /// The optional unit of the energy value.
    pub unit: Option<EnergyUnit>,
}

impl FromCellValue for MagresConvTol {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) => {
                if arr.is_empty() {
                    return Err(Error::Message("empty array for MagresConvTol".to_string()));
                }
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

impl FromKeyValue for MagresConvTol {
    const KEY_NAME: &'static str = "MAGRES_CONV_TOL";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MagresConvTol {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("MAGRES_CONV_TOL", self.to_cell_value())
    }
}

impl ToCellValue for MagresConvTol {
    fn to_cell_value(&self) -> CellValue<'_> {
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

