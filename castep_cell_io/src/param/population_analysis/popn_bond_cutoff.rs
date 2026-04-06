use crate::units::LengthUnit;
use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_f64;

/// Controls the maximum distance between two atoms for which a bond population
/// will be generated during population analysis.
///
/// Keyword type: Real
///
/// Default: 3.0 Å
///
/// Example:
/// POPN_BOND_CUTOFF : 2.54 ang
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PopnBondCutoff {
    /// The cutoff distance value.
    pub value: f64,
    /// The optional unit of the length value.
    pub unit: Option<LengthUnit>,
}

impl FromCellValue for PopnBondCutoff {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) => {
                if arr.is_empty() {
                    return Err(Error::Message("empty array for PopnBondCutoff".to_string()));
                }
                let val = value_as_f64(&arr[0])?;
                let unit = if arr.len() > 1 {
                    Some(LengthUnit::from_cell_value(&arr[1])?)
                } else {
                    None
                };
                Ok(Self { value: val, unit })
            }
            _ => {
                let val = value_as_f64(value)?;
                Ok(Self { value: val, unit: None })
            }
        }
    }
}

impl FromKeyValue for PopnBondCutoff {
    const KEY_NAME: &'static str = "POPN_BOND_CUTOFF";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for PopnBondCutoff {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("POPN_BOND_CUTOFF", self.to_cell_value())
    }
}

impl ToCellValue for PopnBondCutoff {
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
        let val = CellValue::Float(2.54);
        let result = PopnBondCutoff::from_cell_value(&val).unwrap();
        assert_eq!(result.value, 2.54);
        assert!(result.unit.is_none());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(PopnBondCutoff::KEY_NAME, "POPN_BOND_CUTOFF");
    }
}
