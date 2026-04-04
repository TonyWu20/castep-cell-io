use crate::units::TimeUnit;
use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::row_as_f64_n;

/// Determines the time step for a molecular dynamics calculation.
///
/// Keyword type: Real
///
/// Default: 1.0 fs
///
/// Example:
/// MD_DELTA_T : 1.54 fs
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MdDeltaT {
    /// The time step value.
    pub value: f64,
    /// The optional unit of time.
    pub unit: Option<TimeUnit>,
}

impl FromCellValue for MdDeltaT {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(_) => {
                let arr = row_as_f64_n::<2>(value)?;
                Ok(Self {
                    value: arr[0],
                    unit: if arr[1] > 0.0 {
                        Some(TimeUnit::from_cell_value(&CellValue::Float(arr[1]))?)
                    } else {
                        None
                    },
                })
            }
            CellValue::Float(f) => Ok(Self {
                value: *f,
                unit: None,
            }),
            _ => Err(Error::Message("expected float or array".to_string())),
        }
    }
}

impl FromKeyValue for MdDeltaT {
    const KEY_NAME: &'static str = "MD_DELTA_T";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MdDeltaT {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_DELTA_T", self.to_cell_value())
    }
}

impl ToCellValue for MdDeltaT {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value_float_only() {
        let val = CellValue::Float(1.54);
        let result = MdDeltaT::from_cell_value(&val).unwrap();
        assert_eq!(result.value, 1.54);
        assert!(result.unit.is_none());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(MdDeltaT::KEY_NAME, "MD_DELTA_T");
    }
}

