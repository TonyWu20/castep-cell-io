use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::query::value_as_str;
use castep_cell_io::Error;

/// Determines the parallelization strategy used.
///
/// Keyword type: String
///
/// Default: Default
///
/// Example:
/// DATA_DISTRIBUTION : Gvector
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum DataDistribution {
    KPoint,
    GVector,
    Mixed,
    Default,
}

impl FromCellValue for DataDistribution {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "kpoint" => Ok(Self::KPoint),
            "gvector" => Ok(Self::GVector),
            "mixed" => Ok(Self::Mixed),
            "default" => Ok(Self::Default),
            other => Err(Error::Message(format!("unknown DataDistribution: {other}"))),
        }
    }
}

impl FromKeyValue for DataDistribution {
    const KEY_NAME: &'static str = "DATA_DISTRIBUTION";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for DataDistribution {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("DATA_DISTRIBUTION", self.to_cell_value())
    }
}

impl ToCellValue for DataDistribution {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                DataDistribution::KPoint => "Kpoint",
                DataDistribution::GVector => "Gvector",
                DataDistribution::Mixed => "Mixed",
                DataDistribution::Default => "Default",
            }
            .to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_io::CellValue;

    #[test]
    fn test_case_insensitive() {
        assert_eq!(DataDistribution::from_cell_value(&CellValue::Str("kpoint")).unwrap(), DataDistribution::KPoint);
        assert_eq!(DataDistribution::from_cell_value(&CellValue::Str("KPOINT")).unwrap(), DataDistribution::KPoint);
        assert_eq!(DataDistribution::from_cell_value(&CellValue::Str("gvector")).unwrap(), DataDistribution::GVector);
        assert_eq!(DataDistribution::from_cell_value(&CellValue::Str("GVECTOR")).unwrap(), DataDistribution::GVector);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(DataDistribution::from_cell_value(&CellValue::Str("mixed")).unwrap(), DataDistribution::Mixed);
        assert_eq!(DataDistribution::from_cell_value(&CellValue::Str("default")).unwrap(), DataDistribution::Default);
    }

    #[test]
    fn test_invalid() {
        assert!(DataDistribution::from_cell_value(&CellValue::Str("invalid")).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(DataDistribution::KEY_NAME, "DATA_DISTRIBUTION");
    }
}

