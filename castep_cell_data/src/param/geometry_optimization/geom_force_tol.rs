use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_f64;
use serde::{Deserialize, Serialize};
use crate::units::ForceUnit;

/// Controls the tolerance for accepting convergence of the ionic force.
///
/// Keyword type: Real
///
/// Default: 0.05 eV/Å
///
/// Example:
/// GEOM_FORCE_TOL : 0.07 ev/ang
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "GEOM_FORCE_TOL")]
pub struct GeomForceTol {
    /// The force tolerance value.
    pub value: f64,
    /// The optional unit of the force value.
    pub unit: Option<ForceUnit>,
}

impl FromCellValue for GeomForceTol {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) => {
                let value = value_as_f64(&arr[0])?;
                let unit = if arr.len() > 1 {
                    Some(ForceUnit::from_cell_value(&arr[1])?)
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

impl FromKeyValue for GeomForceTol {
    const KEY_NAME: &'static str = "GEOM_FORCE_TOL";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for GeomForceTol {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("GEOM_FORCE_TOL", self.to_cell_value())
    }
}

impl ToCellValue for GeomForceTol {
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


