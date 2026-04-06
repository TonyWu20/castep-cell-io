use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult};
use castep_cell_fmt::query::value_as_f64;
use serde::{Deserialize, Serialize};

/// Determines the mixing amplitude for the charge density in the density mixing procedure.
///
/// Keyword type: Real
///
/// Default: 0.8
///
/// Example:
/// MIX_CHARGE_AMP : 0.5
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "MIX_CHARGE_AMP")]
pub struct MixChargeAmp(pub f64);

impl Default for MixChargeAmp {
    fn default() -> Self {
        Self(0.8)
    }
}

impl FromCellValue for MixChargeAmp {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for MixChargeAmp {
    const KEY_NAME: &'static str = "MIX_CHARGE_AMP";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MixChargeAmp {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("MIX_CHARGE_AMP", CellValue::Float(self.0))
    }
}

impl ToCellValue for MixChargeAmp {
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
        let result = MixChargeAmp::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 0.5);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(MixChargeAmp::KEY_NAME, "MIX_CHARGE_AMP");
    }
}

