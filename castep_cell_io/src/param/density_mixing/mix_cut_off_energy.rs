use crate::units::EnergyUnit;
use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_f64;
use serde::{Deserialize, Serialize};

/// Determines the cutoff energy for the densities used in the density mixing scheme.
///
/// Keyword type: Real
///
/// Default: The value of CUT_OFF_ENERGY
///
/// Example:
/// MIX_CUT_OFF_ENERGY : 250.0 eV
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "MIX_CUT_OFF_ENERGY")]
pub struct MixCutOffEnergy {
    /// The cutoff energy value.
    pub value: f64,
    /// The optional unit of the energy value.
    pub unit: Option<EnergyUnit>,
}

impl FromCellValue for MixCutOffEnergy {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Float(f) => Ok(Self { value: *f, unit: None }),
            CellValue::Array(arr) => {
                if arr.is_empty() {
                    return Err(Error::Message("empty array for MixCutOffEnergy".to_string()));
                }
                let val = value_as_f64(&arr[0])?;
                let unit = if arr.len() > 1 {
                    Some(EnergyUnit::from_cell_value(&arr[1])?)
                } else {
                    None
                };
                Ok(Self { value: val, unit })
            }
            _ => Err(Error::Message("expected float or array for MixCutOffEnergy".to_string())),
        }
    }
}

impl FromKeyValue for MixCutOffEnergy {
    const KEY_NAME: &'static str = "MIX_CUT_OFF_ENERGY";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MixCutOffEnergy {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("MIX_CUT_OFF_ENERGY", self.to_cell_value())
    }
}

impl ToCellValue for MixCutOffEnergy {
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

