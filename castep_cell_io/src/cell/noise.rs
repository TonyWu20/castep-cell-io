use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_f64;
use crate::units::LengthUnit;

/// A physical amount expressed as a value plus an optional unit.
///
/// 6.11 reads the noise keywords with `io_freeform_physical(_, 'L', _)`,
/// i.e. a length value with an optional unit. When no unit is given, CASTEP
/// assumes angstroms. Both noise keywords share this shape.
fn parse_noise_amount(value: &CellValue<'_>) -> CResult<f64> {
    match value {
        CellValue::Float(f) => Ok(*f),
        CellValue::Array(arr) => {
            if arr.is_empty() {
                return Err(Error::Message("empty array for noise amount".to_string()));
            }
            value_as_f64(&arr[0])
        }
        _ => Err(Error::Message(
            "expected a float or [value, unit] array for noise".to_string(),
        )),
    }
}

fn parse_noise_unit(value: &CellValue<'_>) -> CResult<Option<LengthUnit>> {
    match value {
        CellValue::Array(arr) if arr.len() > 1 => Ok(Some(LengthUnit::from_cell_value(&arr[1])?)),
        _ => Ok(None),
    }
}

/// Random noise added onto each ion's position.
///
/// 6.11 keyword type: physical length, no default value beyond `0.0`
/// (no noise). See `cell_random_noise` in `Fundamental/cell.f90`.
///
/// Example:
/// POSITIONS_NOISE : 0.1 ang
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PositionsNoise {
    /// The magnitude of the positional noise.
    pub value: f64,
    /// The unit of the value. `None` means angstroms (the CASTEP default).
    pub unit: Option<LengthUnit>,
}

impl Default for PositionsNoise {
    /// No noise: value `0.0` with the default unit.
    fn default() -> Self {
        Self {
            value: 0.0,
            unit: None,
        }
    }
}

impl FromCellValue for PositionsNoise {
    fn from_cell_value(raw: &CellValue<'_>) -> CResult<Self> {
        let value = parse_noise_amount(raw)?;
        let unit = parse_noise_unit(raw)?;
        Ok(Self { value, unit })
    }
}

impl FromKeyValue for PositionsNoise {
    const KEY_NAME: &'static str = "POSITIONS_NOISE";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for PositionsNoise {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("POSITIONS_NOISE", self.to_cell_value())
    }
}

impl ToCellValue for PositionsNoise {
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

/// Random noise added onto the cell parameters.
///
/// 6.11 keyword type: physical length, no default value beyond `0.0`
/// (no noise). Pairs with [`PositionsNoise`] in `cell_random_noise`.
///
/// Example:
/// CELL_NOISE : 0.05 ang
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellNoise {
    /// The magnitude of the cell noise.
    pub value: f64,
    /// The unit of the value. `None` means angstroms (the CASTEP default).
    pub unit: Option<LengthUnit>,
}

impl Default for CellNoise {
    /// No noise: value `0.0` with the default unit.
    fn default() -> Self {
        Self {
            value: 0.0,
            unit: None,
        }
    }
}

impl FromCellValue for CellNoise {
    fn from_cell_value(raw: &CellValue<'_>) -> CResult<Self> {
        let value = parse_noise_amount(raw)?;
        let unit = parse_noise_unit(raw)?;
        Ok(Self { value, unit })
    }
}

impl FromKeyValue for CellNoise {
    const KEY_NAME: &'static str = "CELL_NOISE";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for CellNoise {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("CELL_NOISE", self.to_cell_value())
    }
}

impl ToCellValue for CellNoise {
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
    fn test_default_is_no_noise() {
        let p = PositionsNoise::default();
        assert_eq!(p.value, 0.0);
        assert_eq!(p.unit, None);
        let c = CellNoise::default();
        assert_eq!(c.value, 0.0);
        assert_eq!(c.unit, None);
    }

    #[test]
    fn test_scalar_value_no_unit() {
        let p = PositionsNoise::from_cell_value(&CellValue::Float(0.1)).unwrap();
        assert_eq!(p.value, 0.1);
        assert_eq!(p.unit, None);
    }

    #[test]
    fn test_value_with_unit() {
        let p = PositionsNoise::from_cell_value(&CellValue::Array(vec![
            CellValue::Float(0.1),
            CellValue::Str("nm"),
        ]))
        .unwrap();
        assert_eq!(p.value, 0.1);
        assert_eq!(p.unit, Some(LengthUnit::Nanometer));
    }

    #[test]
    fn test_empty_array_errors() {
        assert!(PositionsNoise::from_cell_value(&CellValue::Array(vec![])).is_err());
    }

    #[test]
    fn test_positions_noise_key_name() {
        assert_eq!(PositionsNoise::KEY_NAME, "POSITIONS_NOISE");
    }

    #[test]
    fn test_cell_noise_key_name() {
        assert_eq!(CellNoise::KEY_NAME, "CELL_NOISE");
    }

    #[test]
    fn test_positions_noise_to_cell() {
        let p = PositionsNoise {
            value: 0.1,
            unit: None,
        };
        let cell = p.to_cell();
        match cell {
            Cell::KeyValue(name, CellValue::Array(arr)) => {
                assert_eq!(name, "POSITIONS_NOISE");
                assert_eq!(arr.len(), 2);
            }
            _ => panic!("Expected KeyValue with Array, got {:?}", cell),
        }
    }

    #[test]
    fn test_cell_noise_to_cell_with_unit() {
        let c = CellNoise {
            value: 0.5,
            unit: Some(LengthUnit::Ang),
        };
        let cell = c.to_cell();
        match cell {
            Cell::KeyValue(name, CellValue::Array(arr)) => {
                assert_eq!(name, "CELL_NOISE");
                assert_eq!(arr.len(), 2);
                assert!(matches!(arr[1], CellValue::Str(_)));
            }
            _ => panic!("Expected KeyValue with Array, got {:?}", cell),
        }
    }
}
