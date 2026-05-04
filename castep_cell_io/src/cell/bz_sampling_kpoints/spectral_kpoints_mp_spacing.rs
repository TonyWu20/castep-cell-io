use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_f64;
use crate::units::InvLengthUnit;

/// Specifies the spacing of k-points in the Monkhorst-Pack grid for spectral sampling.
///
/// Keyword type: Real
///
/// Default: 0.1 1/ang
///
/// Example:
/// SPECTRAL_KPOINTS_MP_SPACING : 0.05 1/ang
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SpectralKpointsMpSpacing {
    /// The spacing value.
    pub value: f64,
    /// The unit of the spacing value.
    pub unit: Option<InvLengthUnit>,
}

impl FromCellValue for SpectralKpointsMpSpacing {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Float(f) => Ok(Self { value: *f, unit: None }),
            CellValue::Array(arr) => {
                if arr.is_empty() {
                    return Err(Error::Message(
                        "empty array for SpectralKpointsMpSpacing".to_string(),
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
                "expected float or array for SpectralKpointsMpSpacing".to_string(),
            )),
        }
    }
}

impl FromKeyValue for SpectralKpointsMpSpacing {
    const KEY_NAME: &'static str = "SPECTRAL_KPOINT_MP_SPACING";
    const KEY_ALIASES: &'static [&'static str] = &["SPECTRAL_KPOINTS_MP_SPACING"];

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for SpectralKpointsMpSpacing {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue(
            "SPECTRAL_KPOINT_MP_SPACING",
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

impl ToCellValue for SpectralKpointsMpSpacing {
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
    fn test_spectral_kpoints_mp_spacing_scalar() {
        let value = CellValue::Float(0.05);
        let result = SpectralKpointsMpSpacing::from_cell_value(&value).unwrap();
        assert_eq!(result.value, 0.05);
        assert_eq!(result.unit, None);
    }

    #[test]
    fn test_spectral_kpoints_mp_spacing_with_unit() {
        let value = CellValue::Array(vec![
            CellValue::Float(0.05),
            CellValue::Str("1/ang"),
        ]);
        let result = SpectralKpointsMpSpacing::from_cell_value(&value).unwrap();
        assert_eq!(result.value, 0.05);
        assert_eq!(result.unit, Some(InvLengthUnit::Angstrom));
    }

    #[test]
    fn test_spectral_kpoints_mp_spacing_empty_array() {
        let value = CellValue::Array(vec![]);
        let result = SpectralKpointsMpSpacing::from_cell_value(&value);
        assert!(result.is_err());
    }

    #[test]
    fn test_spectral_kpoints_mp_spacing_to_cell_scalar() {
        let spacing = SpectralKpointsMpSpacing {
            value: 0.05,
            unit: None,
        };
        let cell = spacing.to_cell();
        match cell {
            Cell::KeyValue(name, CellValue::Array(arr)) => {
                assert_eq!(name, "SPECTRAL_KPOINT_MP_SPACING");
                assert_eq!(arr.len(), 2);
            }
            _ => panic!("Expected Cell::KeyValue with Array"),
        }
    }

    #[test]
    fn test_spectral_kpoints_mp_spacing_to_cell_with_unit() {
        let spacing = SpectralKpointsMpSpacing {
            value: 0.05,
            unit: Some(InvLengthUnit::Angstrom),
        };
        let cell = spacing.to_cell();
        match cell {
            Cell::KeyValue(name, CellValue::Array(arr)) => {
                assert_eq!(name, "SPECTRAL_KPOINT_MP_SPACING");
                assert_eq!(arr.len(), 2);
            }
            _ => panic!("Expected Cell::KeyValue with Array"),
        }
    }

    #[test]
    fn test_spectral_kpoints_mp_spacing_round_trip() {
        let original = SpectralKpointsMpSpacing {
            value: 0.075,
            unit: Some(InvLengthUnit::NanoMeter),
        };
        let cell_value = CellValue::Array(vec![
            CellValue::Float(0.075),
            CellValue::Str("1/nm"),
        ]);
        let parsed = SpectralKpointsMpSpacing::from_cell_value(&cell_value).unwrap();
        assert_eq!(parsed.value, original.value);
        assert_eq!(parsed.unit, original.unit);
    }
}
