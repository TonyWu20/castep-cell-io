use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_str;

/// Selects the method used by CASTEP for the evaluation of the quantum-mechanical position operator.
///
/// Keyword type: String
///
/// Default: MagresMethod::Crystal
///
/// Example:
/// MAGRES_METHOD : molecular
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MagresMethod {
    /// Uses the reciprocal space representation; applicable for both crystals and "molecule in a box"
    Crystal,
    /// Applicable only for "molecule in a box"; faster calculation
    Molecular,
}

impl Default for MagresMethod {
    fn default() -> Self {
        Self::Crystal
    }
}

impl FromCellValue for MagresMethod {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "crystal" => Ok(Self::Crystal),
            "molecular" => Ok(Self::Molecular),
            other => Err(Error::Message(format!("unknown MagresMethod: {other}"))),
        }
    }
}

impl FromKeyValue for MagresMethod {
    const KEY_NAME: &'static str = "MAGRES_METHOD";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MagresMethod {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("MAGRES_METHOD", self.to_cell_value())
    }
}

impl ToCellValue for MagresMethod {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::String(
            match self {
                MagresMethod::Crystal => "Crystal",
                MagresMethod::Molecular => "Molecular",
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
        assert_eq!(MagresMethod::from_cell_value(&CellValue::Str("crystal")).unwrap(), MagresMethod::Crystal);
        assert_eq!(MagresMethod::from_cell_value(&CellValue::Str("CRYSTAL")).unwrap(), MagresMethod::Crystal);
        assert_eq!(MagresMethod::from_cell_value(&CellValue::Str("molecular")).unwrap(), MagresMethod::Molecular);
    }

    #[test]
    fn test_invalid() {
        assert!(MagresMethod::from_cell_value(&CellValue::Str("invalid")).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(MagresMethod::KEY_NAME, "MAGRES_METHOD");
    }
}
