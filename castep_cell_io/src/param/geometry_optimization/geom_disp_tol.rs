use crate::units::LengthUnit;
use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_f64;
use serde::{Deserialize, Serialize};

/// Determines the tolerance for accepting convergence of the ionic displacement.
///
/// Keyword type: Real
///
/// Default: 0.001 Å
///
/// Example:
/// GEOM_DISP_TOL : 0.002 ang
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "GEOM_DISP_TOL")]
pub struct GeomDispTol {
    /// The displacement tolerance value.
    pub value: f64,
    /// The optional unit of the length value.
    pub unit: Option<LengthUnit>,
}

impl FromCellValue for GeomDispTol {
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

impl FromKeyValue for GeomDispTol {
    const KEY_NAME: &'static str = "GEOM_DISP_TOL";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for GeomDispTol {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("GEOM_DISP_TOL", self.to_cell_value())
    }
}

impl ToCellValue for GeomDispTol {
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
        let val = CellValue::Float(0.002);
        let result = GeomDispTol::from_cell_value(&val).unwrap();
        assert_eq!(result.value, 0.002);
        assert!(result.unit.is_none());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(GeomDispTol::KEY_NAME, "GEOM_DISP_TOL");
    }
}

