use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_str;
use serde::{Deserialize, Serialize};

/// Controls the search method used to locate a transition state.
///
/// Keyword type: String
///
/// Default: TssearchMethod::Lstqst
///
/// Example:
/// TSSEARCH_METHOD : LSTQST
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "TSSEARCH_METHOD")]
pub enum TssearchMethod {
    /// Identifies a transition state using the LSTQST method.
    #[serde(alias = "LSTQST", alias = "lstqst")]
    Lstqst,
    /// Confirms a transition state using the nudged elastic band method.
    #[serde(alias = "NEB", alias = "neb")]
    Neb,
}

impl Default for TssearchMethod {
    fn default() -> Self {
        Self::Lstqst
    }
}

impl FromCellValue for TssearchMethod {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "lstqst" => Ok(Self::Lstqst),
            "neb" => Ok(Self::Neb),
            other => Err(Error::Message(format!("unknown TssearchMethod: {other}"))),
        }
    }
}

impl FromKeyValue for TssearchMethod {
    const KEY_NAME: &'static str = "TSSEARCH_METHOD";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for TssearchMethod {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("TSSEARCH_METHOD", self.to_cell_value())
    }
}

impl ToCellValue for TssearchMethod {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                TssearchMethod::Lstqst => "LSTQST",
                TssearchMethod::Neb => "NEB",
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
        let val = CellValue::Str("lstqst");
        assert_eq!(TssearchMethod::from_cell_value(&val).unwrap(), TssearchMethod::Lstqst);

        let val = CellValue::Str("LSTQST");
        assert_eq!(TssearchMethod::from_cell_value(&val).unwrap(), TssearchMethod::Lstqst);

        let val = CellValue::Str("LstQst");
        assert_eq!(TssearchMethod::from_cell_value(&val).unwrap(), TssearchMethod::Lstqst);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(
            TssearchMethod::from_cell_value(&CellValue::Str("lstqst")).unwrap(),
            TssearchMethod::Lstqst
        );
        assert_eq!(
            TssearchMethod::from_cell_value(&CellValue::Str("neb")).unwrap(),
            TssearchMethod::Neb
        );
    }

    #[test]
    fn test_invalid_variant() {
        let val = CellValue::Str("invalid");
        assert!(TssearchMethod::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(TssearchMethod::KEY_NAME, "TSSEARCH_METHOD");
    }

    #[test]
    fn test_default() {
        assert_eq!(TssearchMethod::default(), TssearchMethod::Lstqst);
    }

    #[test]
    fn test_to_cell_value() {
        assert_eq!(
            TssearchMethod::Lstqst.to_cell_value(),
            CellValue::String("LSTQST".to_string())
        );
        assert_eq!(
            TssearchMethod::Neb.to_cell_value(),
            CellValue::String("NEB".to_string())
        );
    }
}
