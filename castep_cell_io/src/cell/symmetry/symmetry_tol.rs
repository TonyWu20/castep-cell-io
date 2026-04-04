use crate::units::LengthUnit;
use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromCellValue, FromKeyValue, CResult, query::value_as_f64};

/// Controls the tolerance within which symmetry will be considered to be satisfied.
/// If an ion is found within this distance of its symmetric position, the symmetry
/// will be considered to be satisfied. Unit of length must be specified.
///
/// Keyword type: Real
///
/// Default: 0.01 ang
///
/// Example:
/// SYMMETRY_TOL : 0.25 ang
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SymmetryTol {
    /// The tolerance value.
    pub value: f64,
    /// The unit of length for the tolerance.
    pub unit: LengthUnit,
}

impl FromCellValue for SymmetryTol {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() == 2 => {
                Ok(SymmetryTol {
                    value: value_as_f64(&arr[0])?,
                    unit: LengthUnit::from_cell_value(&arr[1])?,
                })
            }
            _ => Err(castep_cell_fmt::Error::Message(
                "SymmetryTol must be an array of [value, unit]".into(),
            )),
        }
    }
}

impl FromKeyValue for SymmetryTol {
    const KEY_NAME: &'static str = "SYMMETRY_TOL";
    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for SymmetryTol {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue(
            "SYMMETRY_TOL",
            CellValue::Array(vec![
                CellValue::Float(self.value),
                self.unit.to_cell_value(),
            ]),
        )
    }
}

impl ToCellValue for SymmetryTol {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(vec![
            CellValue::Float(self.value),
            self.unit.to_cell_value(),
        ])
    }
}


