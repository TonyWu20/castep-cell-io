use castep_cell_fmt::{CellValue, ToCellValue};
use castep_cell_fmt::parse::FromCellValue;
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_string;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default,
)]
/// This keyword specifies the units in which quadrupole moments will be reported.
///
/// Keyword type: String
///
/// Default: barn
///
/// Example:
/// SPECIES_Q_UNIT : fm2
pub enum QuadrupoleMomentUnit {
    #[default]
    #[serde(alias = "BARN", alias = "barn")]
    Barn,
    #[serde(alias = "FM2", alias = "fm2")]
    Fm2,
}

impl FromCellValue for QuadrupoleMomentUnit {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_string(value)?.to_ascii_lowercase().as_str() {
            "barn" => Ok(Self::Barn),
            "fm2" => Ok(Self::Fm2),
            other => Err(Error::Message(format!(
                "unknown QuadrupoleMomentUnit: {other}"
            ))),
        }
    }
}

impl ToCellValue for QuadrupoleMomentUnit {
    fn to_cell_value(&self) -> CellValue<'_> {
        match self {
            QuadrupoleMomentUnit::Barn => CellValue::String("barn".into()),
            QuadrupoleMomentUnit::Fm2 => CellValue::String("fm2".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value_barn_lowercase() {
        let value = CellValue::Str("barn");
        let result = QuadrupoleMomentUnit::from_cell_value(&value);
        assert_eq!(result.unwrap(), QuadrupoleMomentUnit::Barn);
    }

    #[test]
    fn test_from_cell_value_barn_uppercase() {
        let value = CellValue::Str("BARN");
        let result = QuadrupoleMomentUnit::from_cell_value(&value);
        assert_eq!(result.unwrap(), QuadrupoleMomentUnit::Barn);
    }

    #[test]
    fn test_from_cell_value_fm2_lowercase() {
        let value = CellValue::Str("fm2");
        let result = QuadrupoleMomentUnit::from_cell_value(&value);
        assert_eq!(result.unwrap(), QuadrupoleMomentUnit::Fm2);
    }

    #[test]
    fn test_from_cell_value_fm2_uppercase() {
        let value = CellValue::Str("FM2");
        let result = QuadrupoleMomentUnit::from_cell_value(&value);
        assert_eq!(result.unwrap(), QuadrupoleMomentUnit::Fm2);
    }

    #[test]
    fn test_from_cell_value_invalid() {
        let value = CellValue::Str("invalid");
        let result = QuadrupoleMomentUnit::from_cell_value(&value);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_cell_value_barn() {
        let unit = QuadrupoleMomentUnit::Barn;
        let cell_value = unit.to_cell_value();
        match cell_value {
            CellValue::String(s) => assert_eq!(s, "barn"),
            _ => panic!("Expected CellValue::String"),
        }
    }

    #[test]
    fn test_to_cell_value_fm2() {
        let unit = QuadrupoleMomentUnit::Fm2;
        let cell_value = unit.to_cell_value();
        match cell_value {
            CellValue::String(s) => assert_eq!(s, "fm2"),
            _ => panic!("Expected CellValue::String"),
        }
    }

    #[test]
    fn test_round_trip_barn() {
        let original = QuadrupoleMomentUnit::Barn;
        let cell_value = original.to_cell_value();
        // Verify serialization produces the expected string
        match cell_value {
            CellValue::String(s) => {
                // Parse it back from a Str variant
                let parsed = QuadrupoleMomentUnit::from_cell_value(&CellValue::Str(&s)).unwrap();
                assert_eq!(original, parsed);
            }
            _ => panic!("Expected CellValue::String"),
        }
    }

    #[test]
    fn test_round_trip_fm2() {
        let original = QuadrupoleMomentUnit::Fm2;
        let cell_value = original.to_cell_value();
        // Verify serialization produces the expected string
        match cell_value {
            CellValue::String(s) => {
                // Parse it back from a Str variant
                let parsed = QuadrupoleMomentUnit::from_cell_value(&CellValue::Str(&s)).unwrap();
                assert_eq!(original, parsed);
            }
            _ => panic!("Expected CellValue::String"),
        }
    }

    #[test]
    fn test_default_is_barn() {
        assert_eq!(QuadrupoleMomentUnit::default(), QuadrupoleMomentUnit::Barn);
    }
}
