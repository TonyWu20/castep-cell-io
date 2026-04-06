use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_str;

/// Selects which method to use for the position operator in TDDFT.
///
/// Keyword type: String
///
/// Default: TddftPositionMethod::Molecular
///
/// Example:
/// TDDFT_POSITION_METHOD : CRYSTAL
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[derive(Default)]
pub enum TddftPositionMethod {
    /// Appropriate for the "molecule in a box" geometry
    #[default]
    Molecular,
    /// The only option applicable for calculating optical matrix elements for true periodic solids
    Crystal,
}


impl FromCellValue for TddftPositionMethod {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "molecular" => Ok(Self::Molecular),
            "crystal" => Ok(Self::Crystal),
            other => Err(Error::Message(format!("unknown TddftPositionMethod: {other}"))),
        }
    }
}

impl FromKeyValue for TddftPositionMethod {
    const KEY_NAME: &'static str = "TDDFT_POSITION_METHOD";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for TddftPositionMethod {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("TDDFT_POSITION_METHOD", self.to_cell_value())
    }
}

impl ToCellValue for TddftPositionMethod {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::String(
            match self {
                TddftPositionMethod::Molecular => "MOLECULAR",
                TddftPositionMethod::Crystal => "CRYSTAL",
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
    fn test_case_insensitive_parsing() {
        let val = CellValue::Str("molecular");
        assert_eq!(TddftPositionMethod::from_cell_value(&val).unwrap(), TddftPositionMethod::Molecular);

        let val = CellValue::Str("MOLECULAR");
        assert_eq!(TddftPositionMethod::from_cell_value(&val).unwrap(), TddftPositionMethod::Molecular);

        let val = CellValue::Str("Molecular");
        assert_eq!(TddftPositionMethod::from_cell_value(&val).unwrap(), TddftPositionMethod::Molecular);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(
            TddftPositionMethod::from_cell_value(&CellValue::Str("crystal")).unwrap(),
            TddftPositionMethod::Crystal
        );
    }

    #[test]
    fn test_invalid_variant() {
        let val = CellValue::Str("invalid");
        assert!(TddftPositionMethod::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(TddftPositionMethod::KEY_NAME, "TDDFT_POSITION_METHOD");
    }

    #[test]
    fn test_default() {
        assert_eq!(TddftPositionMethod::default(), TddftPositionMethod::Molecular);
    }

    #[test]
    fn test_to_cell_value() {
        assert_eq!(
            TddftPositionMethod::Molecular.to_cell_value(),
            CellValue::String("MOLECULAR".to_string())
        );
        assert_eq!(
            TddftPositionMethod::Crystal.to_cell_value(),
            CellValue::String("CRYSTAL".to_string())
        );
    }

    #[test]
    fn test_to_cell() {
        let cell = TddftPositionMethod::Molecular.to_cell();
        match cell {
            Cell::KeyValue(key, _) => assert_eq!(key, "TDDFT_POSITION_METHOD"),
            _ => panic!("Expected KeyValue cell"),
        }
    }
}
