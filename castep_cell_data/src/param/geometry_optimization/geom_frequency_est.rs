use crate::units::FrequencyUnit;
use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_f64;
use serde::{Deserialize, Serialize};

/// Provides an estimate of the average phonon frequency at the gamma point.
///
/// Keyword type: Real
///
/// Default: 50 THz
///
/// Example:
/// GEOM_FREQUENCY_EST : 17.54 THz
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "GEOM_FREQUENCY_EST")]
pub struct GeomFrequencyEst {
    /// The frequency estimate value.
    pub value: f64,
    /// The optional unit of the frequency value.
    pub unit: Option<FrequencyUnit>,
}

impl FromCellValue for GeomFrequencyEst {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) => {
                let value = value_as_f64(&arr[0])?;
                let unit = if arr.len() > 1 {
                    Some(FrequencyUnit::from_cell_value(&arr[1])?)
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

impl FromKeyValue for GeomFrequencyEst {
    const KEY_NAME: &'static str = "GEOM_FREQUENCY_EST";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for GeomFrequencyEst {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("GEOM_FREQUENCY_EST", self.to_cell_value())
    }
}

impl ToCellValue for GeomFrequencyEst {
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


