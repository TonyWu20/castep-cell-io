use crate::units::InvLengthUnit;
use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_f64;
use serde::{Deserialize, Serialize};

/// Determines the maximum g-vector at which the charge density is mixed.
///
/// Keyword type: Real
///
/// Default: 1.5 Å^-1
///
/// Example:
/// MIX_CHARGE_GMAX : 0.89 1/ang
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "MIX_CHARGE_GMAX")]
pub struct MixChargeGmax {
    /// The maximum g-vector value.
    pub value: f64,
    /// The optional unit of the inverse length value.
    pub unit: Option<InvLengthUnit>,
}

impl FromCellValue for MixChargeGmax {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Float(f) => Ok(Self { value: *f, unit: None }),
            CellValue::Array(arr) => {
                if arr.is_empty() {
                    return Err(Error::Message("empty array for MixChargeGmax".to_string()));
                }
                let val = value_as_f64(&arr[0])?;
                let unit = if arr.len() > 1 {
                    Some(InvLengthUnit::from_cell_value(&arr[1])?)
                } else {
                    None
                };
                Ok(Self { value: val, unit })
            }
            _ => Err(Error::Message("expected float or array for MixChargeGmax".to_string())),
        }
    }
}

impl FromKeyValue for MixChargeGmax {
    const KEY_NAME: &'static str = "MIX_CHARGE_GMAX";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MixChargeGmax {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MIX_CHARGE_GMAX", self.to_cell_value())
    }
}

impl ToCellValue for MixChargeGmax {
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


