use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::{value_as_f64, row_as_f64_n};
use serde::{Deserialize, Serialize};
use crate::units::EnergyUnit;

/// Specifies the cutoff energy for the plane wave basis sets.
///
/// Keyword type: Real
///
/// Default: The value associated with the FINE level of accuracy for the pseudopotentials.
///
/// Example:
/// CUT_OFF_ENERGY : 125 eV
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "CUT_OFF_ENERGY")]
pub struct CutOffEnergy {
    /// The cutoff energy value.
    pub value: f64,
    /// The unit of the energy value.
    pub unit: Option<EnergyUnit>,
}

impl FromCellValue for CutOffEnergy {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Float(f) => Ok(Self { value: *f, unit: None }),
            CellValue::Array(arr) => {
                if arr.is_empty() {
                    return Err(Error::Message("empty array for CutOffEnergy".to_string()));
                }
                let val = value_as_f64(&arr[0])?;
                let unit = if arr.len() > 1 {
                    Some(EnergyUnit::from_cell_value(&arr[1])?)
                } else {
                    None
                };
                Ok(Self { value: val, unit })
            }
            _ => Err(Error::Message("expected float or array for CutOffEnergy".to_string())),
        }
    }
}

impl FromKeyValue for CutOffEnergy {
    const KEY_NAME: &'static str = "CUT_OFF_ENERGY";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for CutOffEnergy {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue(
            "CUT_OFF_ENERGY",
            CellValue::Array(vec![
                CellValue::Float(self.value),
                self.unit
                    .as_ref()
                    .map(|u| u.to_cell_value())
                    .unwrap_or(CellValue::Null),
            ]),
        )
    }
}

impl ToCellValue for CutOffEnergy {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(vec![
            CellValue::Float(self.value),
            self.unit
                .as_ref()
                .map(|u| u.to_cell_value())
                .unwrap_or(CellValue::Null),
        ])
    }
}


