use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult};
use castep_cell_fmt::query::value_as_f64;

/// N parameter for OBS dispersion correction.
///
/// Keyword type: Real
///
/// Default: 0.0
///
/// Example:
/// SEDC_N_OBS : 0.0
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct SedcNObs(pub f64);

impl FromCellValue for SedcNObs {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for SedcNObs {
    const KEY_NAME: &'static str = "SEDC_N_OBS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for SedcNObs {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("SEDC_N_OBS", CellValue::Float(self.0))
    }
}

impl ToCellValue for SedcNObs {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Float(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Float(0.0);
        let result = SedcNObs::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 0.0);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(SedcNObs::KEY_NAME, "SEDC_N_OBS");
    }
}
