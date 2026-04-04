use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_f64;
use serde::{Deserialize, Serialize};
use crate::units::LengthUnit;

/// Specifies the amplitude of the ionic perturbation for finite displacement phonons.
///
/// Keyword type: Real
///
/// Default: 0.01 Bohr
///
/// Example:
/// PHONON_FINITE_DISP : 0.01 ANG
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "PHONON_FINITE_DISP")]
pub struct PhononFiniteDisp {
    /// The displacement amplitude value.
    pub value: f64,
    /// The optional unit of the length value.
    pub unit: Option<LengthUnit>,
}

impl FromCellValue for PhononFiniteDisp {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) => {
                let value = value_as_f64(&arr[0])?;
                let unit = if arr.len() > 1 {
                    Some(LengthUnit::from_cell_value(&arr[1])?)
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

impl FromKeyValue for PhononFiniteDisp {
    const KEY_NAME: &'static str = "PHONON_FINITE_DISP";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for PhononFiniteDisp {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PHONON_FINITE_DISP", self.to_cell_value())
    }
}

impl ToCellValue for PhononFiniteDisp {
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
        let val = CellValue::Float(0.01);
        let result = PhononFiniteDisp::from_cell_value(&val).unwrap();
        assert_eq!(result.value, 0.01);
        assert!(result.unit.is_none());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(PhononFiniteDisp::KEY_NAME, "PHONON_FINITE_DISP");
    }
}

