use crate::units::InvLengthUnit;
use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_f64;
use serde::{Deserialize, Serialize};

/// Determines the maximum g-vector at which the spin density is mixed.
///
/// Keyword type: Real
///
/// Default: 1.5 Å^-1
///
/// Example:
/// MIX_SPIN_GMAX : 0.89 1/ang
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "MIX_SPIN_GMAX")]
pub struct MixSpinGmax {
    /// The maximum g-vector value.
    pub value: f64,
    /// The optional unit of the inverse length value.
    pub unit: Option<InvLengthUnit>,
}

impl FromCellValue for MixSpinGmax {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Float(f) => Ok(Self { value: *f, unit: None }),
            CellValue::Array(arr) => {
                if arr.is_empty() {
                    return Err(Error::Message("empty array for MixSpinGmax".to_string()));
                }
                let val = value_as_f64(&arr[0])?;
                let unit = if arr.len() > 1 {
                    Some(InvLengthUnit::from_cell_value(&arr[1])?)
                } else {
                    None
                };
                Ok(Self { value: val, unit })
            }
            _ => Err(Error::Message("expected float or array for MixSpinGmax".to_string())),
        }
    }
}

impl FromKeyValue for MixSpinGmax {
    const KEY_NAME: &'static str = "MIX_SPIN_GMAX";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MixSpinGmax {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("MIX_SPIN_GMAX", self.to_cell_value())
    }
}

impl ToCellValue for MixSpinGmax {
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

