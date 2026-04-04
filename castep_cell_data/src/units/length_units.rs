use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;
use serde::{Deserialize, Serialize};

/// Specifies the units in which lengths will be reported.
///
/// Keyword type: String
///
/// Default: ang
///
/// Example:
/// LENGTH_UNIT : bohr
#[derive(
    Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub enum LengthUnit {
    #[serde(alias = "BOHR", alias = "bohr")]
    Bohr,
    #[serde(alias = "a0", alias = "A0")]
    BohrA0,
    #[serde(alias = "M", alias = "m")]
    Meter,
    #[serde(alias = "CM", alias = "cm")]
    Centimeter,
    #[serde(alias = "NM", alias = "nm")]
    Nanometer,
    #[default]
    #[serde(alias = "ANG", alias = "ang")]
    Ang,
}

impl FromCellValue for LengthUnit {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "bohr" | "a0" => Ok(Self::Bohr),
            "m" => Ok(Self::Meter),
            "cm" => Ok(Self::Centimeter),
            "nm" => Ok(Self::Nanometer),
            "ang" => Ok(Self::Ang),
            other => Err(Error::Message(format!("unknown LengthUnit: {other}"))),
        }
    }
}

impl FromKeyValue for LengthUnit {
    const KEY_NAME: &'static str = "LENGTH_UNIT";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCellValue for LengthUnit {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                LengthUnit::Bohr => "bohr",
                LengthUnit::BohrA0 => "a0",
                LengthUnit::Meter => "m",
                LengthUnit::Centimeter => "cm",
                LengthUnit::Nanometer => "nm",
                LengthUnit::Ang => "ang",
            }
            .to_string(),
        )
    }
}

impl ToCell for LengthUnit {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("LENGTH_UNIT", self.to_cell_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_insensitive_parsing() {
        assert_eq!(LengthUnit::from_cell_value(&CellValue::Str("bohr")).unwrap(), LengthUnit::Bohr);
        assert_eq!(LengthUnit::from_cell_value(&CellValue::Str("BOHR")).unwrap(), LengthUnit::Bohr);
        assert_eq!(LengthUnit::from_cell_value(&CellValue::Str("ang")).unwrap(), LengthUnit::Ang);
        assert_eq!(LengthUnit::from_cell_value(&CellValue::Str("ANG")).unwrap(), LengthUnit::Ang);
    }

    #[test]
    fn test_bohr_alias() {
        assert_eq!(LengthUnit::from_cell_value(&CellValue::Str("a0")).unwrap(), LengthUnit::Bohr);
        assert_eq!(LengthUnit::from_cell_value(&CellValue::Str("A0")).unwrap(), LengthUnit::Bohr);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(LengthUnit::from_cell_value(&CellValue::Str("m")).unwrap(), LengthUnit::Meter);
        assert_eq!(LengthUnit::from_cell_value(&CellValue::Str("cm")).unwrap(), LengthUnit::Centimeter);
        assert_eq!(LengthUnit::from_cell_value(&CellValue::Str("nm")).unwrap(), LengthUnit::Nanometer);
    }

    #[test]
    fn test_invalid_variant() {
        assert!(LengthUnit::from_cell_value(&CellValue::Str("invalid")).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(LengthUnit::KEY_NAME, "LENGTH_UNIT");
    }

    #[test]
    fn test_default() {
        assert_eq!(LengthUnit::default(), LengthUnit::Ang);
    }

    #[test]
    fn test_to_cell_value() {
        assert_eq!(LengthUnit::Bohr.to_cell_value(), CellValue::String("bohr".to_string()));
        assert_eq!(LengthUnit::Meter.to_cell_value(), CellValue::String("m".to_string()));
        assert_eq!(LengthUnit::Centimeter.to_cell_value(), CellValue::String("cm".to_string()));
        assert_eq!(LengthUnit::Nanometer.to_cell_value(), CellValue::String("nm".to_string()));
        assert_eq!(LengthUnit::Ang.to_cell_value(), CellValue::String("ang".to_string()));
    }

    #[test]
    fn test_to_cell() {
        let cell = LengthUnit::Ang.to_cell();
        match cell {
            Cell::KeyValue(key, _) => assert_eq!(key, "LENGTH_UNIT"),
            _ => panic!("Expected KeyValue cell"),
        }
    }
}
