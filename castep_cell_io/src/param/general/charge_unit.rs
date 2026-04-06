use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_str;

/// Specifies the units in which charges will be reported.
///
/// Keyword type: String
///
/// Default: ChargeUnit::E
///
/// Example:
/// CHARGE_UNIT : c
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[derive(Default)]
pub enum ChargeUnit {
    /// Elementary charge
    #[default]
    E,
    /// Coulomb
    C,
}


impl FromCellValue for ChargeUnit {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "e" => Ok(Self::E),
            "c" => Ok(Self::C),
            other => Err(Error::Message(format!("unknown ChargeUnit: {other}"))),
        }
    }
}

impl FromKeyValue for ChargeUnit {
    const KEY_NAME: &'static str = "CHARGE_UNIT";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for ChargeUnit {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("CHARGE_UNIT", self.to_cell_value())
    }
}

impl ToCellValue for ChargeUnit {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::String(
            match self {
                ChargeUnit::E => "e",
                ChargeUnit::C => "c",
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
        let val = CellValue::Str("e");
        assert_eq!(ChargeUnit::from_cell_value(&val).unwrap(), ChargeUnit::E);

        let val = CellValue::Str("E");
        assert_eq!(ChargeUnit::from_cell_value(&val).unwrap(), ChargeUnit::E);

        let val = CellValue::Str("c");
        assert_eq!(ChargeUnit::from_cell_value(&val).unwrap(), ChargeUnit::C);

        let val = CellValue::Str("C");
        assert_eq!(ChargeUnit::from_cell_value(&val).unwrap(), ChargeUnit::C);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(ChargeUnit::from_cell_value(&CellValue::Str("e")).unwrap(), ChargeUnit::E);
        assert_eq!(ChargeUnit::from_cell_value(&CellValue::Str("c")).unwrap(), ChargeUnit::C);
    }

    #[test]
    fn test_invalid_variant() {
        let val = CellValue::Str("invalid");
        assert!(ChargeUnit::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(ChargeUnit::KEY_NAME, "CHARGE_UNIT");
    }

    #[test]
    fn test_default() {
        assert_eq!(ChargeUnit::default(), ChargeUnit::E);
    }

    #[test]
    fn test_to_cell_value() {
        assert_eq!(
            ChargeUnit::E.to_cell_value(),
            CellValue::String("e".to_string())
        );
        assert_eq!(
            ChargeUnit::C.to_cell_value(),
            CellValue::String("c".to_string())
        );
    }

    #[test]
    fn test_to_cell() {
        let cell = ChargeUnit::E.to_cell();
        match cell {
            Cell::KeyValue(key, _) => assert_eq!(key, "CHARGE_UNIT"),
            _ => panic!("Expected KeyValue cell"),
        }
    }
}
