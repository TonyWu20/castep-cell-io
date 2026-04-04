use crate::units::TimeUnit;
use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::row_as_f64_n;

/// Sets the relevant MD barostat parameters (e.g., Nosé-Hoover barostat mass).
///
/// Keyword type: Real
///
/// Default: 10 × MD_ION_T
///
/// Example:
/// MD_CELL_T : 2 ps
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MdCellT {
    /// The barostat parameter value.
    pub value: f64,
    /// The optional unit of time.
    pub unit: Option<TimeUnit>,
}

impl FromCellValue for MdCellT {
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

impl FromKeyValue for MdCellT {
    const KEY_NAME: &'static str = "MD_CELL_T";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MdCellT {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_CELL_T", self.to_cell_value())
    }
}

impl ToCellValue for MdCellT {
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


