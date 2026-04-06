use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_str;

/// Controls the representation of the nonlocal part of the pseudopotential.
///
/// Keyword type: String
///
/// Default: PspotNonlocalType::Reciprocal
///
/// Example:
/// PSPOT_NONLOCAL_TYPE : REAL
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PspotNonlocalType {
    /// Reciprocal space nonlocal pseudopotentials
    Reciprocal,
    /// Real space nonlocal pseudopotentials
    Real,
}

impl Default for PspotNonlocalType {
    fn default() -> Self {
        Self::Reciprocal
    }
}

impl FromCellValue for PspotNonlocalType {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "reciprocal" => Ok(Self::Reciprocal),
            "real" => Ok(Self::Real),
            other => Err(Error::Message(format!("unknown PspotNonlocalType: {other}"))),
        }
    }
}

impl FromKeyValue for PspotNonlocalType {
    const KEY_NAME: &'static str = "PSPOT_NONLOCAL_TYPE";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for PspotNonlocalType {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("PSPOT_NONLOCAL_TYPE", self.to_cell_value())
    }
}

impl ToCellValue for PspotNonlocalType {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::String(
            match self {
                PspotNonlocalType::Reciprocal => "RECIPROCAL",
                PspotNonlocalType::Real => "REAL",
            }
            .to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_fmt::CellValue;

    #[test]
    fn test_case_insensitive() {
        assert_eq!(PspotNonlocalType::from_cell_value(&CellValue::Str("reciprocal")).unwrap(), PspotNonlocalType::Reciprocal);
        assert_eq!(PspotNonlocalType::from_cell_value(&CellValue::Str("RECIPROCAL")).unwrap(), PspotNonlocalType::Reciprocal);
        assert_eq!(PspotNonlocalType::from_cell_value(&CellValue::Str("real")).unwrap(), PspotNonlocalType::Real);
    }

    #[test]
    fn test_invalid() {
        assert!(PspotNonlocalType::from_cell_value(&CellValue::Str("invalid")).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(PspotNonlocalType::KEY_NAME, "PSPOT_NONLOCAL_TYPE");
    }
}
