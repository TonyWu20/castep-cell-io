use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_str;
use serde::{Deserialize, Serialize};

/// Controls the protocol used for LSTQST transition state searches.
///
/// Keyword type: String
///
/// Default: TssearchLstqstProtocol::CompleteLstqst
///
/// Example:
/// TSSEARCH_LSTQST_PROTOCOL : CompleteLSTQST
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "TSSEARCH_LSTQST_PROTOCOL")]
pub enum TssearchLstqstProtocol {
    /// Uses the LST maximum method.
    #[serde(alias = "LSTMAXIMUM", alias = "lstmaximum")]
    LstMaximum,
    /// Uses the Halgren-Lipscomb method.
    #[serde(alias = "HALGREN-LIPSCOMB", alias = "halgren-lipscomb")]
    HalgrenLipscomb,
    /// Uses the LST/Optimization method.
    #[serde(alias = "LST/OPTIMIZATION", alias = "lst/optimization")]
    LstOptimization,
    /// Uses the complete LSTQST method.
    #[serde(alias = "COMPLETELSTQST", alias = "completelstqst")]
    CompleteLstqst,
    /// Uses the QST/Optimization method.
    #[serde(alias = "QST/OPTIMIZATION", alias = "qst/optimization")]
    QstOptimization,
}

impl Default for TssearchLstqstProtocol {
    fn default() -> Self {
        Self::CompleteLstqst
    }
}

impl FromCellValue for TssearchLstqstProtocol {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "lstmaximum" => Ok(Self::LstMaximum),
            "halgren-lipscomb" => Ok(Self::HalgrenLipscomb),
            "lst/optimization" => Ok(Self::LstOptimization),
            "completelstqst" => Ok(Self::CompleteLstqst),
            "qst/optimization" => Ok(Self::QstOptimization),
            other => Err(Error::Message(format!("unknown TssearchLstqstProtocol: {other}"))),
        }
    }
}

impl FromKeyValue for TssearchLstqstProtocol {
    const KEY_NAME: &'static str = "TSSEARCH_LSTQST_PROTOCOL";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for TssearchLstqstProtocol {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("TSSEARCH_LSTQST_PROTOCOL", self.to_cell_value())
    }
}

impl ToCellValue for TssearchLstqstProtocol {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                TssearchLstqstProtocol::LstMaximum => "LSTMaximum",
                TssearchLstqstProtocol::HalgrenLipscomb => "Halgren-Lipscomb",
                TssearchLstqstProtocol::LstOptimization => "LST/Optimization",
                TssearchLstqstProtocol::CompleteLstqst => "CompleteLSTQST",
                TssearchLstqstProtocol::QstOptimization => "QST/Optimization",
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
        let val = CellValue::Str("completelstqst");
        assert_eq!(
            TssearchLstqstProtocol::from_cell_value(&val).unwrap(),
            TssearchLstqstProtocol::CompleteLstqst
        );

        let val = CellValue::Str("COMPLETELSTQST");
        assert_eq!(
            TssearchLstqstProtocol::from_cell_value(&val).unwrap(),
            TssearchLstqstProtocol::CompleteLstqst
        );

        let val = CellValue::Str("CompleteLstQst");
        assert_eq!(
            TssearchLstqstProtocol::from_cell_value(&val).unwrap(),
            TssearchLstqstProtocol::CompleteLstqst
        );
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(
            TssearchLstqstProtocol::from_cell_value(&CellValue::Str("lstmaximum")).unwrap(),
            TssearchLstqstProtocol::LstMaximum
        );
        assert_eq!(
            TssearchLstqstProtocol::from_cell_value(&CellValue::Str("halgren-lipscomb")).unwrap(),
            TssearchLstqstProtocol::HalgrenLipscomb
        );
        assert_eq!(
            TssearchLstqstProtocol::from_cell_value(&CellValue::Str("lst/optimization")).unwrap(),
            TssearchLstqstProtocol::LstOptimization
        );
        assert_eq!(
            TssearchLstqstProtocol::from_cell_value(&CellValue::Str("completelstqst")).unwrap(),
            TssearchLstqstProtocol::CompleteLstqst
        );
        assert_eq!(
            TssearchLstqstProtocol::from_cell_value(&CellValue::Str("qst/optimization")).unwrap(),
            TssearchLstqstProtocol::QstOptimization
        );
    }

    #[test]
    fn test_invalid_variant() {
        let val = CellValue::Str("invalid");
        assert!(TssearchLstqstProtocol::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(TssearchLstqstProtocol::KEY_NAME, "TSSEARCH_LSTQST_PROTOCOL");
    }

    #[test]
    fn test_default() {
        assert_eq!(TssearchLstqstProtocol::default(), TssearchLstqstProtocol::CompleteLstqst);
    }

    #[test]
    fn test_to_cell_value() {
        assert_eq!(
            TssearchLstqstProtocol::LstMaximum.to_cell_value(),
            CellValue::String("LSTMaximum".to_string())
        );
        assert_eq!(
            TssearchLstqstProtocol::HalgrenLipscomb.to_cell_value(),
            CellValue::String("Halgren-Lipscomb".to_string())
        );
        assert_eq!(
            TssearchLstqstProtocol::LstOptimization.to_cell_value(),
            CellValue::String("LST/Optimization".to_string())
        );
        assert_eq!(
            TssearchLstqstProtocol::CompleteLstqst.to_cell_value(),
            CellValue::String("CompleteLSTQST".to_string())
        );
        assert_eq!(
            TssearchLstqstProtocol::QstOptimization.to_cell_value(),
            CellValue::String("QST/Optimization".to_string())
        );
    }
}
