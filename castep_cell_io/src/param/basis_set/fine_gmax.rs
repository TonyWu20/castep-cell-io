use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_f64;
use serde::{Deserialize, Serialize};
use crate::units::InvLengthUnit;

/// Determines the maximum size of the g-vectors included in the fine grid.
///
/// Keyword type: Real
///
/// Default: -1 1/bohr (results in fine and standard grids being identical)
///
/// Example:
/// FINE_GMAX : 20 1/ang
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "FINE_GMAX")]
pub struct FineGmax {
    /// The maximum g-vector magnitude value.
    pub value: f64,
    /// The unit of the inverse length value.
    pub unit: InvLengthUnit,
}

impl FromCellValue for FineGmax {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) => {
                if arr.is_empty() {
                    return Err(Error::Message("empty array for FineGmax".to_string()));
                }
                let val = value_as_f64(&arr[0])?;
                let unit = if arr.len() > 1 {
                    InvLengthUnit::from_cell_value(&arr[1])?
                } else {
                    InvLengthUnit::default()
                };
                Ok(Self { value: val, unit })
            }
            _ => Err(Error::Message("expected array for FineGmax".to_string())),
        }
    }
}

impl FromKeyValue for FineGmax {
    const KEY_NAME: &'static str = "FINE_GMAX";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for FineGmax {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue(
            "FINE_GMAX",
            CellValue::Array(vec![
                CellValue::Float(self.value),
                self.unit.to_cell_value(),
            ]),
        )
    }
}

impl ToCellValue for FineGmax {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Array(vec![
            CellValue::Float(self.value),
            self.unit.to_cell_value(),
        ])
    }
}


