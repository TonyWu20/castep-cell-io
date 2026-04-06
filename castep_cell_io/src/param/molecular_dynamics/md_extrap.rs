use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_str;

/// Determines the wavefunction extrapolation scheme used for MD.
///
/// Keyword type: String
///
/// Default: MdExtrap::First
///
/// Example:
/// MD_EXTRAP : Mixed
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[derive(Default)]
pub enum MdExtrap {
    /// No extrapolation used
    None,
    /// First order extrapolation
    #[default]
    First,
    /// Second order extrapolation
    Second,
    /// Alternating first and second order extrapolation
    Mixed,
}


impl FromCellValue for MdExtrap {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "none" => Ok(Self::None),
            "first" => Ok(Self::First),
            "second" => Ok(Self::Second),
            "mixed" => Ok(Self::Mixed),
            other => Err(Error::Message(format!("unknown MdExtrap: {other}"))),
        }
    }
}

impl FromKeyValue for MdExtrap {
    const KEY_NAME: &'static str = "MD_EXTRAP";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MdExtrap {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("MD_EXTRAP", self.to_cell_value())
    }
}

impl ToCellValue for MdExtrap {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::String(
            match self {
                MdExtrap::None => "None",
                MdExtrap::First => "First",
                MdExtrap::Second => "Second",
                MdExtrap::Mixed => "Mixed",
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
        assert_eq!(MdExtrap::from_cell_value(&CellValue::Str("none")).unwrap(), MdExtrap::None);
        assert_eq!(MdExtrap::from_cell_value(&CellValue::Str("NONE")).unwrap(), MdExtrap::None);
        assert_eq!(MdExtrap::from_cell_value(&CellValue::Str("first")).unwrap(), MdExtrap::First);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(MdExtrap::from_cell_value(&CellValue::Str("second")).unwrap(), MdExtrap::Second);
        assert_eq!(MdExtrap::from_cell_value(&CellValue::Str("mixed")).unwrap(), MdExtrap::Mixed);
    }

    #[test]
    fn test_invalid() {
        assert!(MdExtrap::from_cell_value(&CellValue::Str("invalid")).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(MdExtrap::KEY_NAME, "MD_EXTRAP");
    }
}

