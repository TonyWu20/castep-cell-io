use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_f64;
use serde::{Deserialize, Serialize};
use crate::units::EnergyUnit;

/// Specifies the derivative of total energy with respect to the natural log
/// of the basis cutoff energy for manual finite basis set correction.
///
/// Keyword type: Real
///
/// Default: 0.0 (but requires a value if FINITE_BASIS_CORR : 1)
///
/// Example:
/// BASIS_DE_DLOGE : -1.2345 eV
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "BASIS_DE_DLOGE")]
pub struct BasisDeDloge {
    /// The derivative value.
    pub value: f64,
    /// The unit of the energy value.
    pub unit: EnergyUnit,
}

impl FromCellValue for BasisDeDloge {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) => {
                if arr.is_empty() {
                    return Err(Error::Message("empty array for BasisDeDloge".to_string()));
                }
                let val = value_as_f64(&arr[0])?;
                let unit = if arr.len() > 1 {
                    EnergyUnit::from_cell_value(&arr[1])?
                } else {
                    EnergyUnit::default()
                };
                Ok(Self { value: val, unit })
            }
            _ => Err(Error::Message("expected array for BasisDeDloge".to_string())),
        }
    }
}

impl FromKeyValue for BasisDeDloge {
    const KEY_NAME: &'static str = "BASIS_DE_DLOGE";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for BasisDeDloge {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue(
            "BASIS_DE_DLOGE",
            CellValue::Array(vec![
                CellValue::Float(self.value),
                self.unit.to_cell_value(),
            ]),
        )
    }
}

impl ToCellValue for BasisDeDloge {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Array(vec![
            CellValue::Float(self.value),
            self.unit.to_cell_value(),
        ])
    }
}


