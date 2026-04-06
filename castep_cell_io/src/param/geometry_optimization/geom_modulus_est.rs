use crate::units::PressureUnit;
use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_f64;
use serde::{Deserialize, Serialize};

/// Provides an estimate of the bulk modulus of the system.
///
/// Keyword type: Real
///
/// Default: 500.0 GPa
///
/// Example:
/// GEOM_MODULUS_EST : 125.4 GPa
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "GEOM_MODULUS_EST")]
pub struct GeomModulusEst {
    /// The modulus estimate value.
    pub value: f64,
    /// The optional unit of the pressure value.
    pub unit: Option<PressureUnit>,
}

impl FromCellValue for GeomModulusEst {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) => {
                let value = value_as_f64(&arr[0])?;
                let unit = if arr.len() > 1 {
                    Some(PressureUnit::from_cell_value(&arr[1])?)
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

impl FromKeyValue for GeomModulusEst {
    const KEY_NAME: &'static str = "GEOM_MODULUS_EST";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for GeomModulusEst {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("GEOM_MODULUS_EST", self.to_cell_value())
    }
}

impl ToCellValue for GeomModulusEst {
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
        let val = CellValue::Float(125.4);
        let result = GeomModulusEst::from_cell_value(&val).unwrap();
        assert_eq!(result.value, 125.4);
        assert!(result.unit.is_none());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(GeomModulusEst::KEY_NAME, "GEOM_MODULUS_EST");
    }
}

