use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_f64;
use serde::{Deserialize, Serialize};
use crate::units::EnergyUnit;

/// Controls the tolerance for accepting convergence of a single eigenvalue or band.
///
/// Keyword type: Real
///
/// Default: 1e-6 eV
///
/// Example:
/// BS_EIGENVALUE_TOL = 1.0e-5 Ha
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "BS_EIGENVALUE_TOL")]
pub struct BsEigenvalueTol {
    /// The eigenvalue tolerance value.
    pub value: f64,
    /// The optional unit of the energy value.
    pub unit: Option<EnergyUnit>,
}

impl FromCellValue for BsEigenvalueTol {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Float(f) => Ok(Self { value: *f, unit: None }),
            CellValue::Array(arr) => {
                if arr.is_empty() {
                    return Err(Error::Message("empty array for BsEigenvalueTol".to_string()));
                }
                let val = value_as_f64(&arr[0])?;
                let unit = if arr.len() > 1 {
                    Some(EnergyUnit::from_cell_value(&arr[1])?)
                } else {
                    None
                };
                Ok(Self { value: val, unit })
            }
            _ => Err(Error::Message("expected float or array for BsEigenvalueTol".to_string())),
        }
    }
}

impl FromKeyValue for BsEigenvalueTol {
    const KEY_NAME: &'static str = "BS_EIGENVALUE_TOL";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for BsEigenvalueTol {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("BS_EIGENVALUE_TOL", self.to_cell_value())
    }
}

impl ToCellValue for BsEigenvalueTol {
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

