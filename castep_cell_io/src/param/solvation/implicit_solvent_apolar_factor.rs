use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult};
use castep_cell_fmt::query::value_as_f64;
use serde::{Deserialize, Serialize};

/// Scaling factor for the apolar contribution to the solvation free energy.
///
/// Keyword type: Real
///
/// Default: 0.0
///
/// Example:
/// IMPLICIT_SOLVENT_APOLAR_FACTOR : 0.0
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "IMPLICIT_SOLVENT_APOLAR_FACTOR")]
pub struct ImplicitSolventApolarFactor(pub f64);

impl FromCellValue for ImplicitSolventApolarFactor {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for ImplicitSolventApolarFactor {
    const KEY_NAME: &'static str = "IMPLICIT_SOLVENT_APOLAR_FACTOR";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for ImplicitSolventApolarFactor {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("IMPLICIT_SOLVENT_APOLAR_FACTOR", CellValue::Float(self.0))
    }
}

impl ToCellValue for ImplicitSolventApolarFactor {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Float(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Float(0.5);
        let result = ImplicitSolventApolarFactor::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 0.5);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(ImplicitSolventApolarFactor::KEY_NAME, "IMPLICIT_SOLVENT_APOLAR_FACTOR");
    }

    #[test]
    fn test_default() {
        let default = ImplicitSolventApolarFactor::default();
        assert_eq!(default.0, 0.0);
    }

    #[test]
    fn test_round_trip() {
        let original = ImplicitSolventApolarFactor(1.5);
        let cell_value = original.to_cell_value();
        let parsed = ImplicitSolventApolarFactor::from_cell_value(&cell_value).unwrap();
        assert_eq!(parsed.0, original.0);
    }
}
