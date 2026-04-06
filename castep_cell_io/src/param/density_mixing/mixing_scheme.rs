use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_str;
use serde::{Deserialize, Serialize};

/// Determines which mixing scheme will be used in the density mixing procedure.
///
/// Keyword type: String
///
/// Default: MixingScheme::Broyden
///
/// Example:
/// MIXING_SCHEME : Pulay
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MIXING_SCHEME")]
#[derive(Default)]
pub enum MixingScheme {
    /// Keriker mixing scheme
    #[serde(alias = "kerker", alias = "KERKER")]
    Kerker,
    /// Linear mixing scheme
    #[serde(alias = "linear", alias = "LINEAR")]
    Linear,
    /// Broyden mixing scheme
    #[serde(alias = "broyden", alias = "BROYDEN")]
    #[default]
    Broyden,
    /// Pulay mixing scheme
    #[serde(alias = "pulay", alias = "PULAY")]
    Pulay,
}


impl FromCellValue for MixingScheme {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "kerker" => Ok(Self::Kerker),
            "linear" => Ok(Self::Linear),
            "broyden" => Ok(Self::Broyden),
            "pulay" => Ok(Self::Pulay),
            other => Err(Error::Message(format!("unknown MixingScheme: {other}"))),
        }
    }
}

impl FromKeyValue for MixingScheme {
    const KEY_NAME: &'static str = "MIXING_SCHEME";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MixingScheme {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("MIXING_SCHEME", self.to_cell_value())
    }
}

impl ToCellValue for MixingScheme {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::String(
            match self {
                MixingScheme::Kerker => "Kerker",
                MixingScheme::Linear => "Linear",
                MixingScheme::Broyden => "Broyden",
                MixingScheme::Pulay => "Pulay",
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
        assert_eq!(MixingScheme::from_cell_value(&CellValue::Str("kerker")).unwrap(), MixingScheme::Kerker);
        assert_eq!(MixingScheme::from_cell_value(&CellValue::Str("KERKER")).unwrap(), MixingScheme::Kerker);
        assert_eq!(MixingScheme::from_cell_value(&CellValue::Str("broyden")).unwrap(), MixingScheme::Broyden);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(MixingScheme::from_cell_value(&CellValue::Str("linear")).unwrap(), MixingScheme::Linear);
        assert_eq!(MixingScheme::from_cell_value(&CellValue::Str("pulay")).unwrap(), MixingScheme::Pulay);
    }

    #[test]
    fn test_invalid() {
        assert!(MixingScheme::from_cell_value(&CellValue::Str("invalid")).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(MixingScheme::KEY_NAME, "MIXING_SCHEME");
    }
}
