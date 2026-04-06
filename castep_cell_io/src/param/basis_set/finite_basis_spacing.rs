use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_f64;
use serde::{Deserialize, Serialize};
use crate::units::EnergyUnit;

/// Determines the spacing of cutoff energies used to estimate the BASIS_DE_DLOGE
/// in automatic calculation of the finite basis set correction.
///
/// Keyword type: Real
///
/// Default: 5.0 eV
///
/// Example:
/// FINITE_BASIS_SPACING : 0.2 Ha
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "FINITE_BASIS_SPACING")]
pub struct FiniteBasisSpacing {
    /// The spacing value.
    pub value: f64,
    /// The unit of the energy value.
    pub unit: EnergyUnit,
}

impl FromCellValue for FiniteBasisSpacing {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) => {
                if arr.is_empty() {
                    return Err(Error::Message("empty array for FiniteBasisSpacing".to_string()));
                }
                let val = value_as_f64(&arr[0])?;
                let unit = if arr.len() > 1 {
                    EnergyUnit::from_cell_value(&arr[1])?
                } else {
                    EnergyUnit::default()
                };
                Ok(Self { value: val, unit })
            }
            _ => Err(Error::Message("expected array for FiniteBasisSpacing".to_string())),
        }
    }
}

impl FromKeyValue for FiniteBasisSpacing {
    const KEY_NAME: &'static str = "FINITE_BASIS_SPACING";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for FiniteBasisSpacing {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue(
            "FINITE_BASIS_SPACING",
            CellValue::Array(vec![
                CellValue::Float(self.value),
                self.unit.to_cell_value(),
            ]),
        )
    }
}

impl ToCellValue for FiniteBasisSpacing {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Array(vec![
            CellValue::Float(self.value),
            self.unit.to_cell_value(),
        ])
    }
}


