use crate::units::TimeUnit;
use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::row_as_f64_n;

/// Sets the relevant MD thermostat parameters (e.g., Nosé-Hoover thermostat mass).
///
/// Keyword type: Real
///
/// Default:
/// 10 × MD_DELTA_T if MD_THERMOSTAT : Nosé-Hoover
/// 100 × MD_DELTA_T if MD_THERMOSTAT : Langevin
///
/// Example:
/// MD_ION_T : 0.5 ps
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MdIonT {
    /// The thermostat parameter value.
    pub value: f64,
    /// The optional unit of time.
    pub unit: Option<TimeUnit>,
}

impl FromCellValue for MdIonT {
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

impl FromKeyValue for MdIonT {
    const KEY_NAME: &'static str = "MD_ION_T";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MdIonT {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("MD_ION_T", self.to_cell_value())
    }
}

impl ToCellValue for MdIonT {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value_float_only() {
        let val = CellValue::Float(0.5);
        let result = MdIonT::from_cell_value(&val).unwrap();
        assert_eq!(result.value, 0.5);
        assert!(result.unit.is_none());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(MdIonT::KEY_NAME, "MD_ION_T");
    }
}

