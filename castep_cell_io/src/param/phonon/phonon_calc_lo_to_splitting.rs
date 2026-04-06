use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_bool;
use serde::{Deserialize, Serialize};

/// Specifies whether to calculate the non-analytical contribution for LO/TO splitting.
///
/// Keyword type: Logical
///
/// Default: true
///
/// Example:
/// PHONON_CALC_LO_TO_SPLITTING : FALSE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "PHONON_CALC_LO_TO_SPLITTING")]
pub struct PhononCalcLoToSplitting(pub bool);

impl FromCellValue for PhononCalcLoToSplitting {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl FromKeyValue for PhononCalcLoToSplitting {
    const KEY_NAME: &'static str = "PHONON_CALC_LO_TO_SPLITTING";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl Default for PhononCalcLoToSplitting {
    fn default() -> Self {
        Self(true) // Default is TRUE
    }
}

impl ToCell for PhononCalcLoToSplitting {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("PHONON_CALC_LO_TO_SPLITTING", CellValue::Bool(self.0))
    }
}

impl ToCellValue for PhononCalcLoToSplitting {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Bool(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value_true() {
        let val = CellValue::Bool(true);
        assert_eq!(PhononCalcLoToSplitting::from_cell_value(&val).unwrap().0, true);
    }

    #[test]
    fn test_from_cell_value_false() {
        let val = CellValue::Bool(false);
        assert_eq!(PhononCalcLoToSplitting::from_cell_value(&val).unwrap().0, false);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(PhononCalcLoToSplitting::KEY_NAME, "PHONON_CALC_LO_TO_SPLITTING");
    }
}

