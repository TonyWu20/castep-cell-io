use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_f64;
use crate::units::InvLengthUnit;

/// Specifies the spacing of k-points along the phonon fine band structure path.
///
/// Keyword type: Real
///
/// Default: 0.1 1/ang
///
/// Example:
/// PHONON_FINE_KPOINT_PATH_SPACING : 0.05 1/ang
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PhononFineKpointPathSpacing {
    /// The spacing value.
    pub value: f64,
    /// The unit of the spacing value.
    pub unit: Option<InvLengthUnit>,
}

impl FromCellValue for PhononFineKpointPathSpacing {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Float(f) => Ok(Self { value: *f, unit: None }),
            CellValue::Array(arr) => {
                if arr.is_empty() {
                    return Err(Error::Message(
                        "empty array for PhononFineKpointPathSpacing".to_string(),
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
                "expected float or array for PhononFineKpointPathSpacing".to_string(),
            )),
        }
    }
}

impl FromKeyValue for PhononFineKpointPathSpacing {
    const KEY_NAME: &'static str = "PHONON_FINE_KPOINT_PATH_SPACING";
    const KEY_ALIASES: &'static [&'static str] = &["PHONON_FINE_KPOINTS_PATH_SPACING"];

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for PhononFineKpointPathSpacing {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue(
            "PHONON_FINE_KPOINT_PATH_SPACING",
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

impl ToCellValue for PhononFineKpointPathSpacing {
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
    fn test_phonon_fine_kpoint_path_spacing_with_unit() {
        let value = CellValue::Array(vec![
            CellValue::Float(0.05),
            CellValue::Str("1/ang"),
        ]);
        let result = PhononFineKpointPathSpacing::from_cell_value(&value).unwrap();
        assert_eq!(result.value, 0.05);
        assert!(result.unit.is_some());
    }
}
