use crate::units::TimeUnit;
use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::row_as_f64_n;

/// Sets the MD thermostat parameter for enhanced MD equilibration.
///
/// Keyword type: Real
///
/// Default: Same as MD_ION_T
///
/// Example:
/// MD_EQM_ION_T : 0.5 ps
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MdEqmIonT {
    /// The equilibration thermostat parameter value.
    pub value: f64,
    /// The optional unit of time.
    pub unit: Option<TimeUnit>,
}

impl FromCellValue for MdEqmIonT {
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

impl FromKeyValue for MdEqmIonT {
    const KEY_NAME: &'static str = "MD_EQM_ION_T";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MdEqmIonT {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_EQM_ION_T", self.to_cell_value())
    }
}

impl ToCellValue for MdEqmIonT {
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


