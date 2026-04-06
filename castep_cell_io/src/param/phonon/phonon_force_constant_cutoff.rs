use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_f64;
use serde::{Deserialize, Serialize};
use crate::units::LengthUnit;

/// Specifies the cutoff for the force constant matrix in a phonon calculation.
///
/// Keyword type: Real
///
/// Default: 0.0
///
/// Example:
/// PHONON_FORCE_CONSTANT_CUTOFF : 6.34 ang
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "PHONON_FORCE_CONSTANT_CUTOFF")]
pub struct PhononForceConstantCutoff {
    /// The cutoff value.
    pub value: f64,
    /// The optional unit of the length value.
    pub unit: Option<LengthUnit>,
}

impl FromCellValue for PhononForceConstantCutoff {
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

impl FromKeyValue for PhononForceConstantCutoff {
    const KEY_NAME: &'static str = "PHONON_FORCE_CONSTANT_CUTOFF";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for PhononForceConstantCutoff {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("PHONON_FORCE_CONSTANT_CUTOFF", self.to_cell_value())
    }
}

impl ToCellValue for PhononForceConstantCutoff {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value_float_only() {
        let val = CellValue::Float(6.34);
        let result = PhononForceConstantCutoff::from_cell_value(&val).unwrap();
        assert_eq!(result.value, 6.34);
        assert!(result.unit.is_none());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(PhononForceConstantCutoff::KEY_NAME, "PHONON_FORCE_CONSTANT_CUTOFF");
    }
}

