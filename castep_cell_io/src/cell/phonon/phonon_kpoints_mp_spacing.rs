use castep_cell_fmt::{Cell, CellValue, CResult, Error, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::query::value_as_f64;

use crate::units::InvLengthUnit;

/// Specifies the spacing of k-points for the Monkhorst-Pack phonon grid.
///
/// Keyword type: Real
///
/// Default: 0.1 1/ang
///
/// Example:
/// PHONON_KPOINT_MP_SPACING : 0.05 1/ang
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PhononKpointsMpSpacing {
    /// The spacing value.
    pub value: f64,
    /// The unit of the spacing value.
    pub unit: Option<InvLengthUnit>,
}

impl FromCellValue for PhononKpointsMpSpacing {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Float(f) => Ok(Self {
                value: *f,
                unit: None,
            }),
            CellValue::Array(arr) => {
                if arr.is_empty() {
                    return Err(Error::Message(
                        "empty array for PhononKpointsMpSpacing".to_string(),
                    ));
                }
                let val = value_as_f64(&arr[0])?;
                let unit = if arr.len() > 1 {
                    Some(InvLengthUnit::from_cell_value(&arr[1])?)
                } else {
                    None
                };
                Ok(Self { value: val, unit })
            }
            _ => Err(Error::Message(
                "expected float or array for PhononKpointsMpSpacing".to_string(),
            )),
        }
    }
}

impl FromKeyValue for PhononKpointsMpSpacing {
    const KEY_NAME: &'static str = "PHONON_KPOINT_MP_SPACING";
    const KEY_ALIASES: &'static [&'static str] = &["PHONON_KPOINTS_MP_SPACING"];

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for PhononKpointsMpSpacing {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue(
            "PHONON_KPOINT_MP_SPACING",
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

impl ToCellValue for PhononKpointsMpSpacing {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Array(vec![
            CellValue::Float(self.value),
            self.unit
                .as_ref()
                .map(|u| u.to_cell_value())
                .unwrap_or(CellValue::Null),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phonon_kpoints_mp_spacing_scalar() {
        let value = CellValue::Float(0.1);
        let result = PhononKpointsMpSpacing::from_cell_value(&value).unwrap();
        assert_eq!(result.value, 0.1);
        assert_eq!(result.unit, None);
    }
}
