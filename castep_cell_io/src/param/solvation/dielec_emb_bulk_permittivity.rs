use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult};
use castep_cell_fmt::query::value_as_f64;
use serde::{Deserialize, Serialize};

/// Bulk permittivity of the dielectric embedding medium.
///
/// Keyword type: Real
///
/// Default: 78.54
///
/// Example:
/// DIELEC_EMB_BULK_PERMITTIVITY : 78.54
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "DIELEC_EMB_BULK_PERMITTIVITY")]
pub struct DielecEmbBulkPermittivity(pub f64);

impl FromCellValue for DielecEmbBulkPermittivity {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_f64(value)?))
    }
}

impl FromKeyValue for DielecEmbBulkPermittivity {
    const KEY_NAME: &'static str = "DIELEC_EMB_BULK_PERMITTIVITY";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for DielecEmbBulkPermittivity {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("DIELEC_EMB_BULK_PERMITTIVITY", CellValue::Float(self.0))
    }
}

impl ToCellValue for DielecEmbBulkPermittivity {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Float(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Float(78.54);
        let result = DielecEmbBulkPermittivity::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 78.54);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(DielecEmbBulkPermittivity::KEY_NAME, "DIELEC_EMB_BULK_PERMITTIVITY");
    }

    #[test]
    fn test_default() {
        let default = DielecEmbBulkPermittivity::default();
        assert_eq!(default.0, 0.0);
    }

    #[test]
    fn test_round_trip() {
        let original = DielecEmbBulkPermittivity(78.54);
        let cell_value = original.to_cell_value();
        let parsed = DielecEmbBulkPermittivity::from_cell_value(&cell_value).unwrap();
        assert_eq!(parsed.0, original.0);
    }
}
