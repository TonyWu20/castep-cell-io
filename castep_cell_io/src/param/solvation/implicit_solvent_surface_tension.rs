use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_f64;
use serde::{Deserialize, Serialize};
use crate::units::ForceConstantUnit;

/// Specifies the surface tension for implicit solvent calculations.
///
/// Keyword type: Real
///
/// Default: 4.7624e-05 Ha/Bohr² (equivalent to 0.0728 N/m for water)
///
/// Example:
/// IMPLICIT_SOLVENT_SURFACE_TENSION : 4.7624e-05 hartree/bohr**2
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "IMPLICIT_SOLVENT_SURFACE_TENSION")]
pub struct ImplicitSolventSurfaceTension {
    /// The surface tension value.
    pub value: f64,
    /// The unit of the surface tension value.
    pub unit: Option<ForceConstantUnit>,
}

impl FromCellValue for ImplicitSolventSurfaceTension {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Float(f) => Ok(Self { value: *f, unit: None }),
            CellValue::Array(arr) => {
                if arr.is_empty() {
                    return Err(Error::Message(
                        "empty array for ImplicitSolventSurfaceTension".to_string(),
                    ));
                }
                let val = value_as_f64(&arr[0])?;
                let unit = if arr.len() > 1 {
                    match &arr[1] {
                        CellValue::Null => None,
                        _ => Some(ForceConstantUnit::from_cell_value(&arr[1])?),
                    }
                } else {
                    None
                };
                Ok(Self { value: val, unit })
            }
            _ => Err(Error::Message(
                "expected float or array for ImplicitSolventSurfaceTension".to_string(),
            )),
        }
    }
}

impl FromKeyValue for ImplicitSolventSurfaceTension {
    const KEY_NAME: &'static str = "IMPLICIT_SOLVENT_SURFACE_TENSION";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for ImplicitSolventSurfaceTension {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue(
            "IMPLICIT_SOLVENT_SURFACE_TENSION",
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

impl ToCellValue for ImplicitSolventSurfaceTension {
    fn to_cell_value(&self) -> CellValue {
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
    fn test_from_cell_value_scalar() {
        let val = CellValue::Float(4.7624e-05);
        let result = ImplicitSolventSurfaceTension::from_cell_value(&val).unwrap();
        assert_eq!(result.value, 4.7624e-05);
        assert_eq!(result.unit, None);
    }

    #[test]
    fn test_from_cell_value_with_unit() {
        let val = CellValue::Array(vec![
            CellValue::Float(0.0728),
            CellValue::String("n/m".to_string()),
        ]);
        let result = ImplicitSolventSurfaceTension::from_cell_value(&val).unwrap();
        assert_eq!(result.value, 0.0728);
        assert_eq!(result.unit, Some(ForceConstantUnit::NewtonPerMeter));
    }

    #[test]
    fn test_key_name() {
        assert_eq!(
            ImplicitSolventSurfaceTension::KEY_NAME,
            "IMPLICIT_SOLVENT_SURFACE_TENSION"
        );
    }

    #[test]
    fn test_round_trip_scalar() {
        let original = ImplicitSolventSurfaceTension {
            value: 4.7624e-05,
            unit: None,
        };
        let cell_value = original.to_cell_value();
        let parsed = ImplicitSolventSurfaceTension::from_cell_value(&cell_value).unwrap();
        assert_eq!(parsed.value, original.value);
        assert_eq!(parsed.unit, None);
    }

    #[test]
    fn test_round_trip_with_unit() {
        let original = ImplicitSolventSurfaceTension {
            value: 0.0728,
            unit: Some(ForceConstantUnit::NewtonPerMeter),
        };
        let cell_value = original.to_cell_value();
        let parsed = ImplicitSolventSurfaceTension::from_cell_value(&cell_value).unwrap();
        assert_eq!(parsed.value, original.value);
        assert_eq!(parsed.unit, original.unit);
    }

    #[test]
    fn test_to_cell() {
        let st = ImplicitSolventSurfaceTension {
            value: 0.0728,
            unit: Some(ForceConstantUnit::NewtonPerMeter),
        };
        let cell = st.to_cell();
        match cell {
            Cell::KeyValue(key, _) => {
                assert_eq!(key, "IMPLICIT_SOLVENT_SURFACE_TENSION");
            }
            _ => panic!("expected KeyValue cell"),
        }
    }
}
