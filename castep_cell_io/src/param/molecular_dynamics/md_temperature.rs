use crate::units::TemperatureUnit;
use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::row_as_f64_n;

/// Determines the temperature for a molecular dynamics calculation (NVT ensemble).
///
/// Keyword type: Real
///
/// Default: 300 K
///
/// Example:
/// MD_TEMPERATURE : 275.4 K
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MdTemperature {
    /// The temperature value.
    pub value: f64,
    /// The optional unit of temperature.
    pub unit: Option<TemperatureUnit>,
}

impl FromCellValue for MdTemperature {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(_) => {
                let arr = row_as_f64_n::<2>(value)?;
                Ok(Self {
                    value: arr[0],
                    unit: if arr[1] > 0.0 {
                        Some(TemperatureUnit::from_cell_value(&CellValue::Float(arr[1]))?)
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

impl FromKeyValue for MdTemperature {
    const KEY_NAME: &'static str = "MD_TEMPERATURE";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MdTemperature {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_TEMPERATURE", self.to_cell_value())
    }
}

impl ToCellValue for MdTemperature {
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
        let val = CellValue::Float(275.4);
        let result = MdTemperature::from_cell_value(&val).unwrap();
        assert_eq!(result.value, 275.4);
        assert!(result.unit.is_none());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(MdTemperature::KEY_NAME, "MD_TEMPERATURE");
    }
}

