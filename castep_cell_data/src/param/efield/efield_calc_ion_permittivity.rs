use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_bool;

/// Controls whether to compute the zero-frequency dielectric permittivity based on ionic response.
///
/// Keyword type: Logical
///
/// Default: true
///
/// Example:
/// EFIELD_CALC_ION_PERMITTIVITY : FALSE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EfieldCalcIonPermittivity(pub bool);

impl Default for EfieldCalcIonPermittivity {
    fn default() -> Self {
        Self(true)
    }
}

impl FromCellValue for EfieldCalcIonPermittivity {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl FromKeyValue for EfieldCalcIonPermittivity {
    const KEY_NAME: &'static str = "EFIELD_CALC_ION_PERMITTIVITY";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for EfieldCalcIonPermittivity {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("EFIELD_CALC_ION_PERMITTIVITY", CellValue::Bool(self.0))
    }
}

impl ToCellValue for EfieldCalcIonPermittivity {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value_true() {
        let val = CellValue::Bool(true);
        assert_eq!(EfieldCalcIonPermittivity::from_cell_value(&val).unwrap().0, true);
    }

    #[test]
    fn test_from_cell_value_false() {
        let val = CellValue::Bool(false);
        assert_eq!(EfieldCalcIonPermittivity::from_cell_value(&val).unwrap().0, false);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(EfieldCalcIonPermittivity::KEY_NAME, "EFIELD_CALC_ION_PERMITTIVITY");
    }
}

